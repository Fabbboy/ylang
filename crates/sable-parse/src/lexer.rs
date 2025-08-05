use std::sync::Arc;

use sable_arena::TypedArena;
use sable_ast::token::{
  Token,
  TokenData,
  TokenError,
  TokenKind,
};
use sable_common::{
  file::source::Source,
  location::Location,
};

const KEYWORDS: phf::Map<&'static str, TokenKind> = phf::phf_map! {
  "func" => TokenKind::Func,
  "var" =>  TokenKind::Var,
};

pub struct Lexer<'src> {
  source: Arc<Source<'src>, &'src TypedArena<Source<'src>>>,

  pos: usize,
  start: usize,

  next: Option<Token<'src>>,
}

impl<'src> Lexer<'src> {
  pub fn new(source: Arc<Source<'src>, &'src TypedArena<Source<'src>>>) -> Self {
    Self {
      source,

      pos: 0,
      start: 0,

      next: None,
    }
  }

  #[inline]
  fn get_char(&self, offset: usize) -> Option<char> {
    self.source.content()[self.pos + offset..].chars().next()
  }

  #[inline]
  fn advance(&mut self) {
    if let Some(c) = self.get_char(0) {
      self.pos += c.len_utf8();
    }
  }

  #[inline]
  fn make_location(&self) -> Location<'src> {
    Location::new(self.start..self.pos, self.source.filename())
  }

  #[inline]
  fn make_lexeme(&self) -> &'src str {
    &self.source.content()[self.start..self.pos]
  }

  #[inline]
  fn make_token(&self, kind: TokenKind, data: Option<TokenData>) -> Token<'src> {
    Token::new(kind, data, self.make_lexeme(), self.make_location())
  }

  #[inline]
  fn check(&self, offset: usize, predicate: impl Fn(char) -> bool) -> bool {
    if let Some(c) = self.get_char(offset) {
      return predicate(c);
    }
    false
  }

  fn skip_trivial(&mut self) -> Option<Token<'src>> {
    loop {
      match self.get_char(0) {
        Some(' ' | '\t' | '\r' | '\n') => self.advance(),
        Some('/') if self.check(1, |c| c == '/') => {
          while let Some(c) = self.get_char(0) {
            if c != '\n' {
              self.advance();
            } else {
              break;
            }
          }
        }
        Some('/') if self.check(1, |c| c == '*') => {
          let comment_start = self.pos;
          self.advance(); // skip '/'
          self.advance(); // skip '*'
          let mut terminated = false;
          while let Some(c) = self.get_char(0) {
            if c == '*' && self.check(1, |c| c == '/') {
              self.advance(); // skip '*'
              self.advance(); // skip '/'
              terminated = true;
              break;
            } else {
              self.advance();
            }
          }
          if !terminated {
            self.start = comment_start;
            return Some(self.make_token(
              TokenKind::Error,
              Some(TokenData::Error(TokenError::UnterminatedComment)),
            ));
          }
        }
        _ => break,
      }
    }
    None
  }

  fn lex_identifier(&mut self) -> Token<'src> {
    while self.get_char(0).is_some() {
      if self.check(0, |c| c.is_ascii_alphanumeric() || c == '_') {
        self.advance();
      } else {
        break;
      }
    }

    let lexeme = self.make_lexeme();
    if let Some(kind) = KEYWORDS.get(lexeme) {
      return self.make_token(*kind, None);
    }

    self.make_token(TokenKind::Identifier, None)
  }

  fn lex_number(&mut self) -> Token<'src> {
    let lex_num = |lexer: &mut Self| {
      while let Some(c) = lexer.get_char(0) {
        if c.is_ascii_digit() {
          lexer.advance();
        } else {
          break;
        }
      }
    };

    lex_num(self);

    if self.check(0, |c| c == '.') && self.check(1, |c| c.is_ascii_digit()) {
      self.advance();
      lex_num(self);

      let lexeme = self.make_lexeme();
      match lexeme.parse::<f64>() {
        Ok(fval) => self.make_token(TokenKind::Float, Some(TokenData::Float(fval))),
        Err(_) => self.make_token(
          TokenKind::Error,
          Some(TokenData::Error(TokenError::InvalidFloat)),
        ),
      }
    } else {
      let lexeme = self.make_lexeme();
      match lexeme.parse::<i64>() {
        Ok(ival) => self.make_token(TokenKind::Integer, Some(TokenData::Integer(ival))),
        Err(_) => self.make_token(
          TokenKind::Error,
          Some(TokenData::Error(TokenError::InvalidInteger)),
        ),
      }
    }
  }

  fn lex(&mut self) -> Token<'src> {
    if let Some(token) = self.skip_trivial() {
      return token;
    }

    self.start = self.pos;
    match self.get_char(0) {
      None => return self.make_token(TokenKind::Eof, None),
      Some(c) => {
        self.advance();
        match c {
          'a'..='z' | 'A'..='Z' | '_' => return self.lex_identifier(),
          '0'..='9' => return self.lex_number(),
          ',' => return self.make_token(TokenKind::Comma, None),
          ';' => return self.make_token(TokenKind::Semicolon, None),
          '+' => return self.make_token(TokenKind::Plus, None),
          '-' => return self.make_token(TokenKind::Minus, None),
          '*' => return self.make_token(TokenKind::Star, None),
          '/' => return self.make_token(TokenKind::Slash, None),
          '=' => return self.make_token(TokenKind::Assign, None),
          '(' => return self.make_token(TokenKind::Paren(true), None),
          ')' => return self.make_token(TokenKind::Paren(false), None),
          '{' => return self.make_token(TokenKind::Brace(true), None),
          '}' => return self.make_token(TokenKind::Brace(false), None),
          ':' => return self.make_token(TokenKind::Colon, None),
          _ => {}
        }
      }
    }

    self.make_token(
      TokenKind::Error,
      Some(TokenData::Error(TokenError::UnknownCharacter)),
    )
  }

  pub fn peek(&mut self) -> Token<'src> {
    if self.next.is_none() {
      self.next = Some(self.lex());
    }
    self.next.clone().unwrap()
  }

  pub fn reset(&mut self) {
    self.pos = 0;
    self.start = 0;
  }
}

impl<'src> Iterator for Lexer<'src> {
  type Item = Token<'src>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.next.is_none() {
      self.next = Some(self.lex());
    }

    let cache = self.next.clone();
    self.next = Some(self.lex());
    cache
  }
}

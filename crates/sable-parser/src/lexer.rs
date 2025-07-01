use std::sync::Arc;

use sable_ast::{
  location::Location,
  token::{Token, TokenError, TokenKind},
  types::PrimitiveType,
};
use sable_common::source::Source;

const KEYWORDS: phf::Map<&'static str, TokenKind> = phf::phf_map! {
  "func" => TokenKind::Func,
  "i32" => TokenKind::Type(PrimitiveType::I32),
};

pub struct Lexer<'ctx> {
  source: Arc<Source<'ctx>>,

  pos: usize,
  start: usize,

  next: Token<'ctx>,
}

impl<'ctx> Lexer<'ctx> {
  pub fn new(source: Arc<Source<'ctx>>) -> Self {
    let mut lexer = Self {
      source,

      pos: 0,
      start: 0,

      next: Token::default(),
    };
    lexer.next = lexer.lex();
    lexer
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
  fn make_location(&self) -> Location {
    Location::new(self.start..self.pos, self.source.filename().clone())
  }

  #[inline]
  fn make_lexeme(&self) -> &'ctx str {
    &self.source.content()[self.start..self.pos]
  }

  #[inline]
  fn make_token(&self, kind: TokenKind) -> Token<'ctx> {
    Token::new(kind, self.make_lexeme(), self.make_location())
  }

  #[inline]
  fn check(&self, offset: usize, predicate: impl Fn(char) -> bool) -> bool {
    if let Some(c) = self.get_char(offset) {
      return predicate(c);
    }
    false
  }

  fn skip_trivial(&mut self) {
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
        _ => break,
      }
    }
  }

  fn lex_identifier(&mut self) -> Token<'ctx> {
    while let Some(_) = self.get_char(0) {
      if self.check(0, |c| c.is_ascii_alphanumeric() || c == '_') {
        self.advance();
      } else {
        break;
      }
    }

    let lexeme = self.make_lexeme();
    if let Some(keyword) = KEYWORDS.get(lexeme) {
      return self.make_token(keyword.clone());
    }

    self.make_token(TokenKind::Identifier)
  }

  fn lex_number(&mut self) -> Token<'ctx> {
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
        Ok(fval) => self.make_token(TokenKind::Float(fval)),
        Err(_) => self.make_token(TokenKind::Error(TokenError::InvalidFloat)),
      }
    } else {
      let lexeme = self.make_lexeme();
      match lexeme.parse::<i64>() {
        Ok(ival) => self.make_token(TokenKind::Integer(ival)),
        Err(_) => self.make_token(TokenKind::Error(TokenError::InvalidInteger)),
      }
    }
  }

  fn lex(&mut self) -> Token<'ctx> {
    self.skip_trivial();

    self.start = self.pos;
    match self.get_char(0) {
      None => return self.make_token(TokenKind::Eof),
      Some(c) => {
        self.advance();
        match c {
          'a'..='z' | 'A'..='Z' | '_' => return self.lex_identifier(),
          '0'..='9' => return self.lex_number(),
          ',' => return self.make_token(TokenKind::Comma),
          ';' => return self.make_token(TokenKind::Semicolon),
          '+' => return self.make_token(TokenKind::Plus),
          '-' => return self.make_token(TokenKind::Minus),
          '*' => return self.make_token(TokenKind::Star),
          '/' => return self.make_token(TokenKind::Slash),
          '=' => return self.make_token(TokenKind::Assign),
          '(' => return self.make_token(TokenKind::Paren(true)),
          ')' => return self.make_token(TokenKind::Paren(false)),
          '{' => return self.make_token(TokenKind::Brace(true)),
          '}' => return self.make_token(TokenKind::Brace(false)),
          _ => {}
        }
      }
    }

    self.make_token(TokenKind::Error(TokenError::UnknownCharacter))
  }

  pub fn peek(&self) -> Token<'ctx> {
    self.next.clone()
  }
}

impl<'ctx> Iterator for Lexer<'ctx> {
  type Item = Token<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    let cache = self.next.clone();
    self.next = self.lex();
    return Some(cache);
  }
}

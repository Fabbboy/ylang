use std::rc::Rc;
use sable_common::Source;

use crate::location::Location;
use crate::token::{Token, TokenType, TokenData};

pub struct Lexer {
    source: Rc<Source>,
    start: usize,
    pos: usize,
}

impl Lexer {
    pub fn new(source: Rc<Source>) -> Self {
        Self { source, start: 0, pos: 0 }
    }

    fn make_location(&self) -> Location {
        Location::new(self.source.clone(), self.start..self.pos)
    }

    fn make_lexeme(&self) -> &str {
        &self.source.content[self.start..self.pos]
    }

    fn make_token(&self, kind: TokenType, data: Option<TokenData>) -> Token {
        Token {
            kind,
            location: self.make_location(),
            lexeme: self.make_lexeme().to_string(),
            data,
        }
    }

    fn get_char(&self, offset: usize) -> Option<char> {
        self.source.content[self.pos + offset..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.get_char(0) {
            self.pos += c.len_utf8();
        }
    }

    fn skip_trivial(&mut self) {
        while let Some(c) = self.get_char(0) {
            match c {
                ' ' | '\t' | '\r' | '\n' => self.advance(),
                _ => break,
            }
        }
    }

    fn lex_identifier(&mut self) -> Token {
        while let Some(c) = self.get_char(0) {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.make_token(TokenType::Identifier, None)
    }

    fn lex_number(&mut self) -> Token {
        let mut has_dot = false;
        while let Some(c) = self.get_char(0) {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !has_dot && self.get_char(c.len_utf8()).map_or(false, |n| n.is_ascii_digit()) {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }
        let lexeme = self.make_lexeme();
        if has_dot {
            match lexeme.parse::<f64>() {
                Ok(f) => self.make_token(TokenType::Float, Some(TokenData::Float(f))),
                Err(_) => self.make_token(TokenType::FloatError, None),
            }
        } else {
            match lexeme.parse::<i64>() {
                Ok(i) => self.make_token(TokenType::Integer, Some(TokenData::Int(i))),
                Err(_) => self.make_token(TokenType::IntegerError, None),
            }
        }
    }

    fn lex(&mut self) -> Token {
        self.skip_trivial();
        self.start = self.pos;
        let ch = match self.get_char(0) {
            Some(c) => c,
            None => return self.make_token(TokenType::Eof, None),
        };
        self.advance();
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(),
            '0'..='9' => self.lex_number(),
            ',' => self.make_token(TokenType::Comma, None),
            ';' => self.make_token(TokenType::Semicolon, None),
            '+' => self.make_token(TokenType::Plus, None),
            '-' => self.make_token(TokenType::Minus, None),
            '*' => self.make_token(TokenType::Star, None),
            '/' => self.make_token(TokenType::Slash, None),
            '=' => self.make_token(TokenType::Assign, None),
            _ => self.make_token(TokenType::Unknown, None),
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.lex();
        if tok.kind == TokenType::Eof {
            None
        } else {
            Some(tok)
        }
    }
}

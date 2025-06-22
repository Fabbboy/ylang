use sable_common::{Source, RangeUsize};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Location {
    pub file: Rc<Source>,
    pub range: RangeUsize,
}

impl Location {
    pub fn new(file: Rc<Source>, range: RangeUsize) -> Self {
        Self { file, range }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Eof,
    Unknown,
    IntegerError,
    FloatError,
    Identifier,
    Integer,
    Float,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
}

#[derive(Debug, Clone)]
pub enum TokenData {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub location: Location,
    pub lexeme: String,
    pub data: Option<TokenData>,
}

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
        Location::new(Rc::clone(&self.source), self.start..self.pos)
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

    pub fn next(&mut self) -> Token {
        self.lex()
    }
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self { Self { lexer } }
    pub fn parse(&mut self) -> ParserStatus { ParserStatus::Ok }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserStatus {
    Ok,
    OhNo,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sable_common::Manager;

    #[test]
    fn lex_simple() {
        let mut manager = Manager::new();
        let src = manager.add_content("a = 1", "test.sable");
        let mut lexer = Lexer::new(src);
        let t1 = lexer.next();
        assert_eq!(t1.kind, TokenType::Identifier);
        let t2 = lexer.next();
        assert_eq!(t2.kind, TokenType::Assign);
    }
}

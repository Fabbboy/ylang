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

use crate::location::Location;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub location: Location,
    pub lexeme: String,
    pub data: Option<TokenData>,
}

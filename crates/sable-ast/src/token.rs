use getset::Getters;

use crate::{location::Location, types::PrimitiveType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenError {
  UnknownCharacter,
  InvalidInteger,
  InvalidFloat,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum TokenKind {
  // Special
  #[default]
  Eof,
  Error(TokenError),

  // Values
  Identifier,
  Integer(i64),
  Float(f64),

  // Brackets
  Paren(bool),
  Brace(bool),

  // Symbols
  Comma,
  Semicolon,

  // Operators
  Plus,
  Minus,
  Star,
  Slash,
  Assign,

  // Keywords
  Func,
  Type(PrimitiveType),
}

impl Eq for TokenKind {}

impl TokenKind {
  pub fn tag(&self) -> TokenKind {
    match self {
      TokenKind::Integer(_) => TokenKind::Integer(0),
      TokenKind::Float(_) => TokenKind::Float(0.0),
      TokenKind::Type(_) => TokenKind::Type(PrimitiveType::I32),
      _ => self.clone(),
    }
  }
}

#[derive(Getters, Default, Clone, Debug)]

pub struct Token<'ctx> {
  #[getset(get = "pub")]
  kind: TokenKind,
  #[getset(get = "pub")]
  lexeme: &'ctx str,
  #[getset(get = "pub")]
  location: Location,
}

impl<'ctx> Token<'ctx> {
  pub fn new(kind: TokenKind, lexeme: &'ctx str, location: Location) -> Self {
    Self {
      kind,
      lexeme,
      location,
    }
  }
}

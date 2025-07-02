use getset::Getters;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
  location::Location,
  types::PrimitiveType,
};

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TokenError {
  UnknownCharacter,
  InvalidInteger,
  InvalidFloat,
}

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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

impl PartialEq for TokenKind {
  fn eq(&self, other: &Self) -> bool {
    std::mem::discriminant(self) == std::mem::discriminant(other)
  }
}

impl TokenKind {
  pub const INTEGER: Self = Self::Integer(0);
  pub const FLOAT: Self = Self::Float(0.0);
  pub const TYPE: Self = Self::Type(PrimitiveType::I32);

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
#[cfg_attr(feature = "serde", derive(Serialize))]
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

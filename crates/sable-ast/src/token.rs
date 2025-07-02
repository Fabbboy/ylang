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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TokenKind {
  // Special
  #[default]
  Eof,
  Error,

  // Values
  Identifier,
  Integer,
  Float,

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
  Type,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TokenData {
  Error(TokenError),
  Integer(i64),
  Float(f64),
  Type(PrimitiveType),
}

#[derive(Getters, Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Token<'ctx> {
  #[getset(get = "pub")]
  kind: TokenKind,
  #[getset(skip)]
  data: Option<TokenData>,
  #[getset(get = "pub")]
  lexeme: &'ctx str,
  #[getset(get = "pub")]
  location: Location,
}

impl<'ctx> Token<'ctx> {
  pub fn new(
    kind: TokenKind,
    data: Option<TokenData>,
    lexeme: &'ctx str,
    location: Location,
  ) -> Self {
    Self {
      kind,
      data,
      lexeme,
      location,
    }
  }

  pub fn data(&self) -> Option<&TokenData> {
    self.data.as_ref()
  }
}

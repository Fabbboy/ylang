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

#[derive(Default, Clone, Debug, PartialEq)]
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


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TokenTag {
  // Special
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

impl Default for TokenTag {
  fn default() -> Self {
    TokenTag::Eof
  }
}

impl TokenKind {
  pub fn tag(&self) -> TokenTag {
    match self {
      TokenKind::Eof => TokenTag::Eof,
      TokenKind::Error(_) => TokenTag::Error,
      TokenKind::Identifier => TokenTag::Identifier,
      TokenKind::Integer(_) => TokenTag::Integer,
      TokenKind::Float(_) => TokenTag::Float,
      TokenKind::Paren(b) => TokenTag::Paren(*b),
      TokenKind::Brace(b) => TokenTag::Brace(*b),
      TokenKind::Comma => TokenTag::Comma,
      TokenKind::Semicolon => TokenTag::Semicolon,
      TokenKind::Plus => TokenTag::Plus,
      TokenKind::Minus => TokenTag::Minus,
      TokenKind::Star => TokenTag::Star,
      TokenKind::Slash => TokenTag::Slash,
      TokenKind::Assign => TokenTag::Assign,
      TokenKind::Func => TokenTag::Func,
      TokenKind::Type(_) => TokenTag::Type,
    }
  }
}

#[derive(Getters, Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Token<'ctx> {
  #[getset(get = "pub")]
  kind: TokenKind,
  #[getset(skip)]
  tag: TokenTag,
  #[getset(get = "pub")]
  lexeme: &'ctx str,
  #[getset(get = "pub")]
  location: Location,
}

impl<'ctx> Token<'ctx> {
  pub fn new(kind: TokenKind, lexeme: &'ctx str, location: Location) -> Self {
    Self {
      tag: kind.tag(),
      kind,
      lexeme,
      location,
    }
  }

  pub fn tag(&self) -> TokenTag {
    self.tag
  }
}

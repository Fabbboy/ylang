use getset::Getters;
use sable_common::location::Location;
#[cfg(feature = "serde")]
use serde::Serialize;

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
  Colon,

  // Operators
  Plus,
  Minus,
  Star,
  Slash,
  Assign,

  // Keywords
  Func,
  Var,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TokenData {
  Error(TokenError),
  Integer(i64),
  Float(f64),
}

#[derive(Getters, Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Token<'ctx> {
  #[getset(get = "pub")]
  kind: TokenKind,
  #[getset(get = "pub")]
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
}

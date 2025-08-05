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
  UnterminatedComment,
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

#[derive(Getters, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Token<'src> {
  #[getset(get = "pub")]
  kind: TokenKind,
  #[getset(get = "pub")]
  data: Option<TokenData>,
  #[getset(get = "pub")]
  lexeme: &'src str,
  #[getset(get = "pub")]
  location: Location<'src>,
}

impl<'src> Token<'src> {
  pub fn new(
    kind: TokenKind,
    data: Option<TokenData>,
    lexeme: &'src str,
    location: Location<'src>,
  ) -> Self {
    Self {
      kind,
      data,
      lexeme,
      location,
    }
  }
}

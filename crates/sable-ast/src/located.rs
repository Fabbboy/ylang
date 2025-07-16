use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

use crate::token::Token;

#[derive(Debug, Clone, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Located<'loc, T> {
  #[getset(get = "pub")]
  value: T,
  #[getset(get = "pub")]
  location: Location<'loc>,
}

impl<'ctx> From<Token<'ctx>> for Located<'ctx, &'ctx str> {
  fn from(token: Token<'ctx>) -> Self {
    Self {
      value: token.lexeme(),
      location: token.location().clone(),
    }
  }
}

impl<'loc, T> From<Located<'loc, T>> for Located<'loc, Box<T>> {
  fn from(located: Located<'loc, T>) -> Self {
    Self {
      value: Box::new(located.value),
      location: located.location,
    }
  }
}

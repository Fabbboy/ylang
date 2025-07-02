use getset::Getters;
use typed_builder::TypedBuilder;

use crate::location::Location;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum LiteralExpression {
  Integer(IntegerExpression),
  Float(FloatExpression),
}

impl LiteralExpression {
  pub fn location(&self) -> &Location {
    match self {
      LiteralExpression::Integer(expr) => expr.location(),
      LiteralExpression::Float(expr) => expr.location(),
    }
  }
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IntegerExpression {
  #[getset(get = "pub")]
  value: i64,
  #[getset(get = "pub")]
  location: Location,
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FloatExpression {
  #[getset(get = "pub")]
  value: f64,
  #[getset(get = "pub")]
  location: Location,
}

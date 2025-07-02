use getset::Getters;
use typed_builder::TypedBuilder;

use crate::location::Location;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum LiteralExpression {
  Integer(IntegerExpression),
  Float(FloatExpression),
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct IntegerExpression {
  #[getset(get = "pub")]
  value: i64,
  #[getset(get = "pub")]
  location: Location,
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FloatExpression {
  #[getset(get = "pub")]
  value: f64,
  #[getset(get = "pub")]
  location: Location,
}

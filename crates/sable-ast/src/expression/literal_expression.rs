use getset::Getters;
use typed_builder::TypedBuilder;

use sable_common::location::Location;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum LiteralExpression<'ctx> {
  Integer(IntegerExpression<'ctx>),
  Float(FloatExpression<'ctx>),
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IntegerExpression<'ctx> {
  #[getset(get = "pub")]
  value: i64,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FloatExpression<'ctx> {
  #[getset(get = "pub")]
  value: f64,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

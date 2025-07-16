use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

use crate::expression::Expression;

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssignExpression<'ctx> {
  #[getset(get = "pub")]
  identifier: &'ctx str,
  #[getset(get = "pub")]
  value: Box<Expression<'ctx>>,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

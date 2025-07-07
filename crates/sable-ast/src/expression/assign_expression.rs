use getset::Getters;
use typed_builder::TypedBuilder;

use crate::{
  expression::Expression,
  location::Location,
};

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssignExpression<'ctx> {
  #[getset(get = "pub")]
  identifier: &'ctx str,
  #[getset(get = "pub")]
  value: Box<Expression<'ctx>>,
  #[getset(get = "pub")]
  location: Location,
}

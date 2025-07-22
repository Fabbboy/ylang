use getset::Getters;
use typed_builder::TypedBuilder;

use crate::{
  expression::Expression,
  located::Located,
};
use sable_common::location::Location;

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssignExpression<'ctx> {
  #[getset(get = "pub")]
  identifier: Located<'ctx, &'ctx str>,
  #[getset(get = "pub")]
  value: &'ctx Expression<'ctx>,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

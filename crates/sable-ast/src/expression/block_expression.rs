use crate::statement::Statement;
use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

#[derive(Getters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BlockExpression<'ctx> {
  #[getset(get = "pub")]
  body: Vec<Statement<'ctx>>,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

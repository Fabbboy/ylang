use crate::statement::Statement;
use getset::{Getters, MutGetters};
use typed_builder::TypedBuilder;

#[derive(Getters, MutGetters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BlockExpression<'ctx> {
  #[getset(get = "pub", get_mut = "pub")]
  body: Vec<Statement<'ctx>>,
}

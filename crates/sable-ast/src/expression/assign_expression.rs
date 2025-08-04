use getset::{
  Getters,
  MutGetters,
};
use typed_builder::TypedBuilder;

use crate::{
  expression::Expression,
  located::Located,
};
use sable_common::interner::Entry;

#[derive(Debug, MutGetters, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssignExpression<'ctx> {
  #[getset(get = "pub")]
  identifier: Located<'ctx, Entry>,
  #[getset(get = "pub", get_mut = "pub")]
  value: &'ctx mut Expression<'ctx>,
}

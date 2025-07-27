use crate::{
  expression::Expression,
  located::Located,
  types::Type,
};
use getset::Getters;
use typed_builder::TypedBuilder;
use sable_common::interner::Entry;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VariableStatement<'ctx> {
  #[getset(get = "pub")]
  name: Located<'ctx, Entry>,
  #[getset(get = "pub")]
  initializer: Expression<'ctx>,
  #[getset(get = "pub")]
  type_: Located<'ctx, Type<'ctx>>,
}

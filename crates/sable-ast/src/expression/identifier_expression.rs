use getset::Getters;
use typed_builder::TypedBuilder;

use crate::located::Located;
use sable_common::interner::Entry;

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IdentifierExpression<'ctx> {
  #[getset(get = "pub")]
  pub name: Located<'ctx, Entry>,
}

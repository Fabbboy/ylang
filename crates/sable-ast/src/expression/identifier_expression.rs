use getset::Getters;
use typed_builder::TypedBuilder;

use sable_common::interner::Entry;

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IdentifierExpression {
  #[getset(get = "pub")]
  pub name: Entry,
}

use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IdentifierExpression<'ctx> {
  #[getset(get = "pub")]
  pub name: &'ctx str,
  #[getset(get = "pub")]
  pub location: Location,
}

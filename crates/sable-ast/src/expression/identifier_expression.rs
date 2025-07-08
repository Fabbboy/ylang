use getset::Getters;
use typed_builder::TypedBuilder;

use crate::location::Location;

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IdentifierExpression<'ctx> {
  #[getset(get = "pub")]
  pub name: &'ctx str,
  #[getset(get = "pub")]
  pub location: Location,
}

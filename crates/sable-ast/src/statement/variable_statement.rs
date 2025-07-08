use crate::{
  expression::Expression,
  types::Type,
};
use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VariableStatement<'ctx> {
  #[getset(get = "pub")]
  name: &'ctx str,
  #[getset(get = "pub")]
  initializer: Expression<'ctx>,
  #[getset(get = "pub")]
  type_: Type<'ctx>,
  #[getset(get = "pub")]
  location: Location,
}

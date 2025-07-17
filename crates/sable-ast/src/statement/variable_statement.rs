use crate::{
  expression::Expression,
  located::Located,
  types::Type,
};
use getset::Getters;
use typed_builder::TypedBuilder;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VariableStatement<'ctx> {
  #[getset(get = "pub")]
  name: Located<'ctx, &'ctx str>,
  #[getset(get = "pub")]
  initializer: Expression<'ctx>,
  #[getset(get = "pub")]
  type_: Located<'ctx, Type<'ctx>>,
}

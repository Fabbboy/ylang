use std::rc::Rc;

use getset::Getters;
use typed_builder::TypedBuilder;

use crate::{
  expression::Expression,
  location::Location,
  types::Type,
};

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VariableStatement<'ctx> {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  initializer: Expression<'ctx>,
  #[getset(get = "pub")]
  type_: Type,
  #[getset(get = "pub")]
  location: Location,
}

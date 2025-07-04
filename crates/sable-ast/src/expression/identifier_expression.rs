use std::rc::Rc;

use getset::Getters;
use typed_builder::TypedBuilder;

use crate::location::Location;

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IdentifierExpression {
  #[getset(get = "pub")]
  pub name: Rc<str>,
  #[getset(get = "pub")]
  pub location: Location,
}

use std::rc::Rc;

use getset::Getters;
use typed_builder::TypedBuilder;

use crate::{
  expression::Expression,
  location::Location,
};

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssignExpression {
  #[getset(get = "pub")]
  identifier: Rc<str>,
  #[getset(get = "pub")]
  value: Box<Expression>,
  #[getset(get = "pub")]
  location: Location,
}

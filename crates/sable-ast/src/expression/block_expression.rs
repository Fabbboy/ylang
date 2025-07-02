use getset::Getters;
use typed_builder::TypedBuilder;

use crate::{
  location::Location,
  statement::Statement,
};

#[derive(Getters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BlockExpression {
  #[getset(get = "pub")]
  body: Vec<Statement>,
  #[getset(get = "pub")]
  location: Location,
}

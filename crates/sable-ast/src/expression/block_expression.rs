use getset::Getters;
use typed_builder::TypedBuilder;

use bumpalo::collections::Vec as BumpVec;

use crate::{
  location::Location,
  statement::Statement,
};

#[derive(Getters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BlockExpression<'ctx> {
  #[getset(get = "pub")]
  body: BumpVec<'ctx, Statement<'ctx>>,
  #[getset(get = "pub")]
  location: Location,
}

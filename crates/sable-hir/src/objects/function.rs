use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use getset::Getters;
use sable_ast::{
  location::Location,
  types::Type,
};
use typed_builder::TypedBuilder;

use crate::statement::HirStatement;

#[derive(TypedBuilder, Debug, Getters)]
pub struct HirParam {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  ty: Type,
  #[getset(get = "pub")]
  location: Location,
}

#[derive(TypedBuilder, Debug, Getters)]
pub struct HirFunction<'hir> {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  params: &'hir [&'hir HirParam],
  #[getset(get = "pub")]
  return_type: Type,
  #[getset(get = "pub")]
  body: BumpVec<'hir, HirStatement>,
}

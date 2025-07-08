use std::rc::Rc;

use getset::Getters;
use sable_ast::{
  location::Location,
  types::Type,
};
use typed_builder::TypedBuilder;

use crate::statement::HirStmt;

#[derive(TypedBuilder, Debug, Getters)]
pub struct HirParam<'hir> {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  ty: Type<'hir>,
  #[getset(get = "pub")]
  location: Location,
}

#[derive(TypedBuilder, Debug, Getters)]
pub struct HirFunction<'hir> {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  params: &'hir [&'hir HirParam<'hir>],
  #[getset(get = "pub")]
  return_type: Type<'hir>,
  #[getset(get = "pub")]
  body: &'hir [HirStmt<'hir>],
}

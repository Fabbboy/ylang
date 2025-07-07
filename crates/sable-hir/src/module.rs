use getset::{
  Getters,
  MutGetters,
};

use bumpalo::collections::Vec as BumpVec;
use sable_common::context::Context;
use typed_builder::TypedBuilder;

use crate::objects::function::HirFunction;

#[derive(Debug, TypedBuilder, Getters, MutGetters)]
pub struct HirModule<'hir> {
  #[getset(get = "pub", get_mut = "pub")]
  funcs: BumpVec<'hir, HirFunction<'hir>>,
}

impl<'hir> HirModule<'hir> {
  pub fn new(ctx: &'hir Context) -> Self {
    Self {
      funcs: BumpVec::new_in(ctx.hir_bump()),
    }
  }
}

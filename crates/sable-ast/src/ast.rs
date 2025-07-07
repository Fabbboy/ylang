use getset::{
  Getters,
  MutGetters,
};
use sable_common::context::Context;

use crate::objects::function::Function;
use bumpalo::{
  Bump,
  collections::Vec as BumpVec,
};

#[derive(Getters, MutGetters, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Ast<'ctx> {
  #[getset(get = "pub")]
  #[cfg_attr(feature = "serde", serde(skip))]
  ast_bump: &'ctx Bump,
  #[getset(get_mut = "pub", get = "pub")]
  funcs: BumpVec<'ctx, Function<'ctx>>,
}

impl<'ctx> Ast<'ctx> {
  pub fn new(ctx: &'ctx Context) -> Self {
    Self {
      ast_bump: ctx.ast_bump(),
      funcs: BumpVec::new_in(ctx.ast_bump()),
    }
  }
}

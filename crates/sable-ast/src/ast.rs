use getset::{
  Getters,
  MutGetters,
};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::objects::function::Function;
use bumpalo::{
  Bump,
  collections::Vec as BumpVec,
};

#[derive(Getters, MutGetters, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Ast<'ctx> {
  #[getset(get = "pub")]
  #[cfg_attr(feature = "serde", serde(skip))]
  ast_bump: &'ctx Bump,
  #[getset(get_mut = "pub", get = "pub")]
  funcs: BumpVec<'ctx, Function<'ctx>>,
}

impl<'ctx> Ast<'ctx> {
  pub fn new(ast_bump: &'ctx Bump) -> Self {
    Self {
      ast_bump,
      funcs: BumpVec::new_in(ast_bump),
    }
  }
}

use getset::{
  Getters,
  MutGetters,
};
#[cfg(feature = "serde")]
use serde::Serialize;

use bumpalo::{
  collections::Vec as BumpVec,
  Bump,
};
use crate::objects::function::Function;

#[derive(Getters, MutGetters, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Ast<'ctx> {
  #[getset(get_mut = "pub", get = "pub")]
  funcs: BumpVec<'ctx, Function>,
}

impl<'ctx> Ast<'ctx> {
  pub fn new(bump: &'ctx Bump) -> Self {
    Self {
      funcs: BumpVec::new_in(bump),
    }
  }
}

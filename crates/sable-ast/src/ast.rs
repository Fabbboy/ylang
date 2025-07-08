use getset::{
  Getters,
  MutGetters,
};
use std::boxed::Box;

use crate::objects::function::Function;
use bumpalo::{
  Bump,
  collections::Vec as BumpVec,
};

#[derive(Getters, MutGetters, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Ast<'ctx> {
  #[cfg_attr(feature = "serde", serde(skip))]
  ast_bump: Box<Bump>,
  #[getset(get_mut = "pub", get = "pub")]
  funcs: BumpVec<'ctx, Function<'ctx>>,
}

impl<'ctx> Ast<'ctx> {
  pub fn new() -> Self {
    let ast_bump = Box::new(Bump::new());
    let bump_ptr: *const Bump = &*ast_bump;
    let funcs = BumpVec::new_in(unsafe { &*bump_ptr });
    Self { ast_bump, funcs }
  }

  pub fn ast_bump(&self) -> &Bump {
    &self.ast_bump
  }
}

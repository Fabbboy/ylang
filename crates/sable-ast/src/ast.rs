use getset::{
  Getters,
  MutGetters,
};

use crate::objects::function::Function;

#[derive(Getters, MutGetters, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Ast<'ctx> {
  #[getset(get_mut = "pub", get = "pub")]
  funcs: Vec<Function<'ctx>>,
}

impl<'ctx> Ast<'ctx> {
  pub fn new() -> Self {
    Ast { funcs: Vec::new() }
  }
}

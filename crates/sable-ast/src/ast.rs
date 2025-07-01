use getset::{
  Getters,
  MutGetters,
};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::objects::function::Function;

#[derive(Getters, MutGetters, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Ast {
  #[getset(get_mut = "pub", get = "pub")]
  funcs: Vec<Function>,
}

impl Ast {
  pub fn new() -> Self {
    Self { funcs: Vec::new() }
  }
}

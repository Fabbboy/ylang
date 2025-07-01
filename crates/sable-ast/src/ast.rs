use getset::{
  Getters,
  MutGetters,
};
use serde::Serialize;

use crate::objects::function::Function;

#[derive(Getters, MutGetters, Debug, Serialize)]
pub struct Ast {
  #[getset(get_mut = "pub", get = "pub")]
  funcs: Vec<Function>,
}

impl Ast {
  pub fn new() -> Self {
    Self { funcs: Vec::new() }
  }
}

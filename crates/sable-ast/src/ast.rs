use getset::{Getters, MutGetters};

use crate::objects::function::Function;

#[derive(Getters, MutGetters, Debug)]
pub struct Ast {
  #[getset(get_mut = "pub", get = "pub")]
  funcs: Vec<Function>,
}

impl Ast {
  pub fn new() -> Self {
    Self { funcs: Vec::new() }
  }
}

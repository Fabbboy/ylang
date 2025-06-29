use getset::Getters;

use crate::objects::function::Function;

#[derive(Getters)]
pub struct Ast {
  #[getset(get = "pub")]
  funcs: Vec<Function>,
}

impl Ast {
  pub fn new() -> Self {
    Self { funcs: Vec::new() }
  }
}

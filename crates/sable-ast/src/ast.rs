use getset::Getters;

#[derive(Getters)]
pub struct Ast {}

impl Ast {
  pub fn new() -> Self {
    Self {}
  }
}

use std::pin::Pin;

use bumpalo::Bump;
use getset::Getters;

#[derive(Getters, Debug)]
pub struct GlobalContext {
  #[getset(get = "pub")]
  hir_bump: Pin<Box<Bump>>,
}

impl GlobalContext {
  pub fn new() -> Self {
    Self {
      hir_bump: Box::pin(Bump::new()),
    }
  }
}

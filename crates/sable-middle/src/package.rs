use std::pin::Pin;

use bumpalo::Bump;
use getset::Getters;
use sable_hir::definition::OwnerId;

#[derive(Debug, Getters)]
pub struct Package {
  #[getset(get = "pub")]
  arena: Pin<Box<Bump>>,
  #[getset(get = "pub")]
  def: OwnerId,
}

impl Package {
  pub fn new(def: OwnerId) -> Self {
    Self {
      arena: Box::pin(Bump::new()),
      def,
    }
  }
}

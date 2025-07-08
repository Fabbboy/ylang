use getset::{
  Getters,
  Setters,
};

use bumpalo::Bump;

use crate::{
  objects::function::HirFunction,
  symbol::SymTable,
};

#[derive(Debug, Getters, Setters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HirModule<'hir> {
  #[cfg_attr(feature = "serde", serde(skip))]
  hir_bump: Box<Bump>,
  #[getset(get = "pub", set = "pub")]
  funcs: &'hir [&'hir HirFunction<'hir>],
  #[getset(get = "pub")]
  symbols: SymTable<'hir>,
}

impl<'hir> HirModule<'hir> {
  pub fn new() -> Self {
    Self {
      hir_bump: Box::new(Bump::new()),
      funcs: &[],
      symbols: SymTable::new(),
    }
  }

  pub fn hir_bump(&self) -> &Bump {
    &self.hir_bump
  }
}

impl<'hir> Default for HirModule<'hir> {
  fn default() -> Self {
    Self::new()
  }
}

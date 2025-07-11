use getset::Getters;
use sable_arena::arena::Arena;

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Context<'hir> {
  #[getset(get = "pub")]
  hir_arena: &'hir Arena,
}

impl<'hir> Context<'hir> {
  pub fn new(hir_arena: &'hir Arena) -> Self {
    Self { hir_arena }
  }
}

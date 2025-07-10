use getset::Getters;
use sable_arena::arena::Arena;
use sable_common::interner::Interner;

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Context<'hir> {
  #[getset(get = "pub")]
  hir_arena: &'hir Arena,
  #[getset(get = "pub")]
  str_map: Interner<&'hir str>,
}

impl<'hir> Context<'hir> {
  pub fn new(hir_arena: &'hir Arena) -> Self {
    Self {
      hir_arena,
      str_map: Interner::new(),
    }
  }
}

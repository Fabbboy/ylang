use getset::Getters;
use indexmap::IndexMap;
use sable_arena::arena::Arena;
use sable_common::interner::Interner;

use crate::ty::TypeId;

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Context<'hir> {
  #[getset(get = "pub")]
  hir_arena: &'hir Arena,
  #[getset(get = "pub")]
  str_map: Interner<&'hir str>,
  #[getset(get = "pub")]
  type_map: IndexMap<TypeId<'hir>, usize>,
}

impl<'hir> Context<'hir> {
  pub fn new(hir_arena: &'hir Arena) -> Self {
    Self {
      hir_arena,
      str_map: Interner::new(),
      type_map: IndexMap::new(),
    }
  }
}

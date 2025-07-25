use getset::{
  Getters,
  MutGetters,
};
use indexmap::IndexSet;
use sable_arena::arena::Arena;

use crate::hir::module::Module;

#[derive(Debug)]
pub struct Symbol(pub usize);

#[derive(Debug, Getters, MutGetters)]
pub struct Package<'hir> {
  #[getset(get = "pub")]
  hir_arena: &'hir Arena,
  #[getset(get_mut = "pub", get = "pub")]
  mods: &'hir mut [Option<Module<'hir>>],
  #[getset(get = "pub")]
  strintern: IndexSet<&'hir str>,
}

impl<'hir> Package<'hir> {
  pub fn new<'ast>(hir_arena: &'hir Arena, trees: usize) -> Self {
    let mods = hir_arena.alloc_slice_with(trees, |_| None);
    Self {
      hir_arena,
      mods,
      strintern: IndexSet::new(),
    }
  }
}

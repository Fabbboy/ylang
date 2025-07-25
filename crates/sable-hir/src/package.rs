use getset::{
  Getters,
  MutGetters,
};
use indexmap::IndexSet;
use sable_arena::arena::Arena;
use sable_ast::ast::Ast;

use crate::hir::module::Module;

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
  pub fn new<'ast>(hir_arena: &'hir Arena, trees: &[Ast<'ast>]) -> Self {
    let mods = hir_arena.alloc_slice_with(trees.len(), |_| None);
    Self {
      hir_arena,
      mods,
      strintern: IndexSet::new(),
    }
  }

  pub fn intern(&mut self, string: &str) -> &str {
    if self.strintern.contains(string) {
      self.strintern.get(string).unwrap()
    } else {
      let copy: &'hir mut str = self.hir_arena.alloc_str(string);
      let (idx, _) = self.strintern.insert_full(copy);
      self.strintern.get_index(idx).unwrap()
    }
  }
}

use getset::{
  Getters,
  MutGetters,
};
use indexmap::IndexSet;
use sable_arena::TypedArena;
use sable_ast::ast::Ast;

use crate::hir::{
  item::Item,
  module::Module,
};

#[derive(Debug, Getters, MutGetters)]
pub struct Package<'hir> {
  #[getset(get = "pub")]
  module_arena: &'hir TypedArena<Module<'hir>>,
  #[getset(get = "pub")]
  item_arena: &'hir TypedArena<Item<'hir>>,
  #[getset(get_mut = "pub", get = "pub")]
  mods: &'hir mut [Option<Module<'hir>>],
  #[getset(get = "pub")]
  strintern: IndexSet<&'hir str>,
}

impl<'hir> Package<'hir> {
  pub fn new<'ast>(
    module_arena: &'hir TypedArena<Module<'hir>>,
    item_arena: &'hir TypedArena<Item<'hir>>,
    trees: &[Ast<'ast>],
  ) -> Self {
    let mods = module_arena
      .as_untyped()
      .alloc_slice_with(trees.len(), |_| None);
    Self {
      module_arena,
      item_arena,
      mods,
      strintern: IndexSet::new(),
    }
  }

  pub fn intern(&mut self, string: &str) -> &str {
    if self.strintern.contains(string) {
      self.strintern.get(string).unwrap()
    } else {
      let copy: &'hir mut str = self.module_arena.alloc_str(string);
      let (idx, _) = self.strintern.insert_full(copy);
      self.strintern.get_index(idx).unwrap()
    }
  }
}

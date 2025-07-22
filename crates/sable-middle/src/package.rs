use getset::{
  Getters,
  MutGetters,
};
use indexmap::IndexSet;
use sable_arena::arena::Arena;
use sable_ast::ast::Ast;

use crate::hir::module::Module;

#[derive(Debug)]
pub struct Symbol(pub usize);

#[derive(Debug, Getters, MutGetters)]
pub struct Package<'ast, 'hir> {
  #[getset(get = "pub")]
  hir_arena: &'hir Arena,
  #[getset(get_mut = "pub", get = "pub")]
  trees: &'hir mut [Option<Ast<'ast>>],
  #[getset(get_mut = "pub", get = "pub")]
  mods: &'hir mut [Option<Module<'hir>>],
  #[getset(get = "pub")]
  strintern: IndexSet<&'hir str>,
}

impl<'ast, 'hir> Package<'ast, 'hir> {
  pub fn new(tree_arena: &'hir Arena, hir_arena: &'hir Arena, trees: usize) -> Self {
    let trees = tree_arena.alloc_slice_with(trees, |_| None);
    let mods = hir_arena.alloc_slice_with(trees.len(), |_| None);
    Package {
      trees,
      mods,
      hir_arena,
      strintern: IndexSet::new(),
    }
  }

  pub fn obtain(&mut self, arena: &'ast Arena, idx: usize) -> Option<&mut Ast<'ast>> {
    if idx >= self.trees.len() {
      return None;
    }
    let slot = &mut self.trees[idx];
    if slot.is_none() {
      *slot = Some(Ast::new(arena));
    }
    slot.as_mut()
  }

  pub fn resolve(&mut self, input: &str) -> Symbol {
    if let Some(idx) = self.strintern.get_index_of(input) {
      Symbol(idx)
    } else {
      let input = self.hir_arena.alloc_str(input);
      let (idx, _) = self.strintern.insert_full(input);
      Symbol(idx)
    }
  }

  pub fn get_str(&self, symbol: Symbol) -> Option<&'hir str> {
    self.strintern.get_index(symbol.0).copied()
  }
}

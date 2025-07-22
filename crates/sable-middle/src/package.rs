use getset::{
  Getters,
  MutGetters,
};
use sable_arena::arena::Arena;
use sable_ast::ast::Ast;

use crate::hir::module::Module;

#[derive(Debug, Getters, MutGetters)]
pub struct Package<'ast, 'hir> {
  #[getset(get = "pub")]
  hir_arena: &'hir Arena,
  #[getset(get_mut = "pub", get = "pub")]
  trees: &'hir mut [Option<Ast<'ast>>],
  #[getset(get_mut = "pub", get = "pub")]
  mods: &'hir mut [Option<Module<'hir>>],
}

impl<'ast, 'hir> Package<'ast, 'hir> {
  pub fn new(tree_arena: &'hir Arena, hir_arena: &'hir Arena, trees: usize) -> Self {
    let trees = tree_arena.alloc_slice_with(trees, |_| None);
    let mods = hir_arena.alloc_slice_with(trees.len(), |_| None);
    Package {
      trees,
      mods,
      hir_arena,
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
}

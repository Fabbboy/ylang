use getset::{
  Getters,
  MutGetters,
};
use sable_arena::arena::Arena;
use sable_ast::ast::Ast;

use crate::hir::module::Module;

#[derive(Debug, Getters, MutGetters)]
pub struct TranslationUnit<'ast, 'middle> {
  #[getset(get_mut = "pub", get = "pub")]
  trees: &'middle mut [Option<Ast<'ast>>],
  #[getset(get_mut = "pub", get = "pub")]
  mods: &'middle mut [Option<Module>],
}

impl<'ast, 'middle> TranslationUnit<'ast, 'middle> {
  pub fn new(tree_arena: &'middle Arena, trees: usize) -> Self {
    let trees = tree_arena.alloc_slice_with(trees, |_| None);
    let mods = tree_arena.alloc_slice_with(trees.len(), |_| None);
    TranslationUnit { trees, mods }
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

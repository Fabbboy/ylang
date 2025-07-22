use std::mem::MaybeUninit;

use getset::{
  Getters,
  MutGetters,
};
use sable_arena::arena::Arena;
use sable_ast::ast::Ast;

#[derive(Debug, Getters, MutGetters)]
pub struct Module<'ast, 'middle> {
  #[getset(get_mut = "pub", get = "pub")]
  trees: &'middle mut [MaybeUninit<Ast<'ast>>],
}

impl<'ast, 'middle> Module<'ast, 'middle> {
  pub fn new(tree_arena: &'middle Arena, trees: usize) -> Self {
    let trees = tree_arena.alloc_slice_with(trees, |_| MaybeUninit::uninit());
    Module { trees }
  }

  pub fn obtain(&mut self, arena: &'ast Arena, idx: usize) -> Option<&mut Ast<'ast>> {
    if idx < self.trees.len() {
      let tree = unsafe { self.trees[idx].as_mut_ptr().as_mut() };
      if let Some(tree_ref) = tree {
        if tree_ref.funcs().is_empty() {
          *tree_ref = Ast::new(arena);
        }
        return Some(tree_ref);
      }
    }
    None
  }
}

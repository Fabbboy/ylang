use getset::{
  Getters,
  MutGetters,
};
use sable_arena::arena::Arena;
use sable_ast::ast::Ast;

#[derive(Debug, Getters, MutGetters)]
pub struct Module<'ast, 'middle> {
  #[getset(get_mut = "pub", get = "pub")]
  trees: &'middle mut [Option<Ast<'ast>>],
}

impl<'ast, 'middle> Module<'ast, 'middle> {
  pub fn new(tree_arena: &'middle Arena, trees: usize) -> Self {
    let trees = tree_arena.alloc_slice_with(trees, |_| None);
    Module { trees }
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

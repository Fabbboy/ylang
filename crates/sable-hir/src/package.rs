use getset::{
  Getters,
  MutGetters,
};
use sable_arena::TypedArena;
use sable_common::interner::StrInterner;

use crate::hir::{
  item::Item,
  module::Module,
};

#[derive(Debug, Getters, MutGetters)]
pub struct Package<'hir> {
  #[getset(get = "pub")]
  item_arena: &'hir TypedArena<Item<'hir>>,
  #[getset(get_mut = "pub", get = "pub")]
  mods: Vec<Module<'hir>>,
  #[getset(get = "pub")]
  strintern: &'hir StrInterner<'hir>,
}

impl<'hir> Package<'hir> {
  pub fn new(
    item_arena: &'hir TypedArena<Item<'hir>>,
    strintern: &'hir StrInterner<'hir>,
  ) -> Self {
    Self {
      item_arena,
      mods: Vec::new(),
      strintern,
    }
  }
}

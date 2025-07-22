use getset::Getters;
use sable_arena::arena::Arena;
use typed_builder::TypedBuilder;

use crate::hir::item::Item;

#[derive(Debug, Getters, TypedBuilder)]
pub struct Module<'hir> {
  #[getset(get = "pub")]
  items: &'hir [Item<'hir>],
  #[getset(get = "pub")]
  arena: &'hir Arena,
}

use getset::Getters;
use sable_arena::TypedArena;
use typed_builder::TypedBuilder;

use crate::hir::item::Item;

#[derive(Debug, Getters, TypedBuilder)]
pub struct Module<'hir> {
  #[getset(get = "pub")]
  items: &'hir [Item<'hir>],
  #[getset(get = "pub")]
  item_arena: &'hir TypedArena<Item<'hir>>,
}

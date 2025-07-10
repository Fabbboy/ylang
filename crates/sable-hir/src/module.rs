use getset::Getters;
use sable_arena::arena::Arena;
use typed_builder::TypedBuilder;

use crate::item::ItemId;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Module<'hir> {
  #[getset(get = "pub")]
  id: ModId,
  #[getset(get = "pub")]
  arena: &'hir Arena,
  #[getset(get = "pub")]
  items: &'hir [ItemId<'hir>],
}

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ModId(pub usize);

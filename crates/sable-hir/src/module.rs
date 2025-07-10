use getset::Getters;
use sable_arena::arena::Arena;
use typed_builder::TypedBuilder;

use crate::item::Item;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Module<'hir> {
  #[getset(get = "pub")]
  id: DefId,
  #[getset(get = "pub")]
  arena: &'hir Arena,
  #[getset(get = "pub")]
  items: &'hir [Item<'hir>],
}

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ModId(pub usize);

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OwnerId(pub usize);

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DefId(pub OwnerId, pub ModId);

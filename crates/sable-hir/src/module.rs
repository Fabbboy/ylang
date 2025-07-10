use getset::Getters;
use sable_arena::arena::Arena;

use crate::item::Item;

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Module<'hir> {
  #[getset(get = "pub")]
  id: ModId,
  #[getset(get = "pub")]
  arena: &'hir Arena,
  #[getset(get = "pub")]
  items: &'hir [&'hir Item],
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ModId(pub usize);

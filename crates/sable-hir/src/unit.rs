use getset::Getters;
use sable_arena::arena::Arena;
use sable_common::location::Location;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ModId(pub usize);

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OwnerId(pub usize);

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

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Item {
  #[getset(get = "pub")]
  id: OwnerId,
  #[getset(get = "pub")]
  kind: ItemKind,
  #[getset(get = "pub")]
  location: Location,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ItemKind {}

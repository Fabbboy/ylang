use getset::Getters;
use sable_arena::arena::Arena;

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Package<'hir> {
  arena: &'hir Arena,
  #[getset(get = "pub")]
  mods: &'hir [&'hir Module<'hir>],
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OwnerId(pub usize);

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Module<'hir> {
  #[getset(get = "pub")]
  mod_id: OwnerId,
  items: &'hir [&'hir Item],
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Item {}

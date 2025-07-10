use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

use crate::{
  module::ModId,
  object::function::HirFunctionId,
};

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DefId(pub ModId, pub usize);

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Item<'hir> {
  #[getset(get = "pub")]
  id: DefId,
  #[getset(get = "pub")]
  kind: ItemKind<'hir>,
  #[getset(get = "pub")]
  location: Location<'hir>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ItemKind<'hir> {
  Func(HirFunctionId<'hir>),
}

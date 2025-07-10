use getset::Getters;
use sable_common::location::Location;

use crate::module::ModId;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DefId(pub ModId, pub usize);

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Item {
  #[getset(get = "pub")]
  id: DefId,
  #[getset(get = "pub")]
  kind: ItemKind,
  #[getset(get = "pub")]
  location: Location,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ItemKind {}

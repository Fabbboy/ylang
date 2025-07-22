use getset::Getters;
use sable_common::location::Location;

#[derive(Debug, Getters)]
pub struct Item<'hir> {
  #[getset(get = "pub")]
  kind: ItemKind,
  #[getset(get = "pub")]
  location: Location<'hir>,
}

#[derive(Debug)]
pub enum ItemKind {}

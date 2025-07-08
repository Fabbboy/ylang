#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OwnerId(pub usize);

#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Definition {
  owner_id: OwnerId,
  def_id: usize,
}

impl Definition {
  pub fn new(owner_id: OwnerId, def_id: usize) -> Self {
    Self { owner_id, def_id }
  }
}

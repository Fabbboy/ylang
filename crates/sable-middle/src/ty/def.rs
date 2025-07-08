#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Definition {
  owner_id: usize,
  def_id: usize,
}

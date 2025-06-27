use std::ops::Range;

use getset::Getters;
use sable_common::FileId;

#[derive(Getters, Default, Clone, Debug, PartialEq, Eq)]
pub struct Location {
  #[getset(get = "pub")]
  range: Range<usize>,
  #[getset(get = "pub")]
  filename: FileId,
}

impl Location {
  pub fn new(range: Range<usize>, filename: FileId) -> Self {
    Self { range, filename }
  }
}

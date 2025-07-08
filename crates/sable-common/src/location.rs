use std::ops::Range;

use getset::Getters;

use crate::file::FileId;

#[derive(Getters, Default, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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

  pub fn merge(&self, other: &Self) -> Option<Self> {
    if self.filename != other.filename {
      return None;
    }

    let start = self.range.start.min(other.range.start);
    let end = self.range.end.max(other.range.end);
    Some(Self::new(start..end, self.filename.clone()))
  }
}

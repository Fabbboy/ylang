use std::ops::Range;

use getset::Getters;

#[derive(Getters, Default, Clone, Debug, PartialEq, Eq)]
pub struct Location<'ctx> {
  #[getset(get = "pub")]
  range: Range<usize>,
  #[getset(get = "pub")]
  filename: &'ctx str,
}

impl<'ctx> Location<'ctx> {
  pub fn new(range: Range<usize>, filename: &'ctx str) -> Self {
    Self { range, filename }
  }
}

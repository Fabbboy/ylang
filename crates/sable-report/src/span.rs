use std::ops::Range;

use getset::Getters;

#[derive(Getters)]
pub struct Span<'ctx> {
  #[getset(get = "pub")]
  range: Range<usize>,
  #[getset(get = "pub")]
  filename: &'ctx str,
}

impl<'ctx> Span<'ctx> {
  pub fn new(range: Range<usize>, filename: &'ctx str) -> Self {
    Self { range, filename }
  }
}

use bumpalo::Bump;
use getset::Getters;

use crate::source::Source;

#[derive(Getters)]
pub struct Manager<'ctx> {
  #[getset(get = "pub")]
  sources: Vec<Source<'ctx>>,
}

impl<'ctx> Manager<'ctx> {
  pub fn new() -> Self {
    Self {
      sources: Vec::new(),
    }
  }

  pub fn add_source(&mut self, source: &str, filename: &str, bump: &'ctx Bump) {
    let source = Source::new(source, filename, bump);
    self.sources.push(source);
  }
}

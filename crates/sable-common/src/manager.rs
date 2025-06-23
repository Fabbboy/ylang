use std::collections::HashMap;

use bumpalo::Bump;
use getset::Getters;

use crate::source::Source;

#[derive(Getters)]
pub struct Manager<'ctx> {
  #[getset(get = "pub")]
  sources: HashMap<&'ctx str, Source<'ctx>>,
}

impl<'ctx> Manager<'ctx> {
  pub fn new() -> Self {
    Self {
      sources: HashMap::new(),
    }
  }

  pub fn add_source(&mut self, source: &str, filename: &'ctx str, bump: &'ctx Bump) {
    let source = Source::new(source, filename, bump);
    self.sources.insert(filename, source);
  }
}

use bumpalo::Bump;

use crate::source::Source;

pub struct Manager<'ctx> {
  sources: Vec<Source<'ctx>>,
}

impl<'ctx> Manager<'ctx> {
  pub fn new() -> Self {
    Self {
      sources: Vec::new(),
    }
  }

  pub fn add_source(&mut self, source: &str, filename: &str, bump: &'ctx Bump) -> &Source<'ctx> {
    let source = Source::new(source, filename, bump);
    self.sources.push(source);
    self.sources.last().unwrap()
  }

  pub fn get_source(&self, index: usize) -> Option<&[Source<'ctx>]> {
    if index < self.sources.len() {
      Some(&self.sources[index..])
    } else {
      None
    }
  }
}

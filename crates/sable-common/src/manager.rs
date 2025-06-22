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

  pub fn add_source(&mut self, source: Source<'ctx>) {
    self.sources.push(source);
  }

  pub fn get_source(&self, index: usize) -> Option<&[Source<'ctx>]> {
    if index < self.sources.len() {
      Some(&self.sources[index..])
    } else {
      None
    }
  }
}

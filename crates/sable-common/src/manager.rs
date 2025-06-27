use std::{collections::HashMap, sync::Arc};

use bumpalo::Bump;
use getset::Getters;

use crate::{FileId, source::Source};

#[derive(Getters)]
pub struct Manager<'ctx> {
  #[getset(get = "pub")]
  sources: HashMap<FileId, Arc<Source<'ctx>>>,
}

impl<'ctx> Manager<'ctx> {
  pub fn new() -> Self {
    Self {
      sources: HashMap::new(),
    }
  }

  pub fn add_source(
    &mut self,
    source: &str,
    filename: &'ctx str,
    bump: &'ctx Bump,
  ) -> Arc<Source<'ctx>> {
    let source = Source::new(source, filename, bump);
    let id = source.filename().clone();
    let source = Arc::new(source);
    self.sources.insert(id, source.clone());
    source
  }
}

use std::{
  collections::HashMap,
  sync::Arc,
};

use getset::Getters;
use sable_arena::arena::Arena;

use crate::file::{
  FileId,
  source::Source,
};

#[derive(Getters)]
pub struct Manager<'src> {
  #[getset(get = "pub")]
  sources: HashMap<FileId, Arc<Source<'src>>>,
  file_bump: &'src Arena,
}

impl<'src> Manager<'src> {
  pub fn new(arena: &'src Arena) -> Self {
    Self {
      sources: HashMap::new(),
      file_bump: arena,
    }
  }

  pub fn add_source(&mut self, source: &str, filename: &str) -> Arc<Source<'src>> {
    let source = Source::new(source, filename, self.file_bump);
    let id = source.filename().clone();
    let source = Arc::new(source);
    self.sources.insert(id, source.clone());
    source
  }
}

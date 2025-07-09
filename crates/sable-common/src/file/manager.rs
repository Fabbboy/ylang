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
  sources: HashMap<FileId, Arc<Source<'src>, &'src Arena>>,
  file_bump: &'src Arena,
}

impl<'src> Manager<'src> {
  pub fn new(arena: &'src Arena) -> Self {
    Self {
      sources: HashMap::new(),
      file_bump: arena,
    }
  }

  pub fn add_source(&mut self, source: &str, filename: &str) -> Arc<Source<'src>, &'src Arena> {
    let source = Source::new(source, filename, self.file_bump);
    let id = source.filename().clone();
    let source = Arc::new_in(source, self.file_bump);
    self.sources.insert(id, source.clone());
    source
  }
}

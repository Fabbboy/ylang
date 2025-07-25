use std::{
  collections::HashMap,
  sync::Arc,
};

use getset::{Getters, MutGetters};
use sable_arena::arena::Arena;

use crate::cache::ErrorCache;

use crate::file::{
  FileId,
  source::Source,
};

#[derive(Getters, MutGetters)]
pub struct Manager<'src> {
  #[getset(get = "pub")]
  sources: HashMap<FileId<'src>, Arc<Source<'src>, &'src Arena>>,
  #[getset(get = "pub", get_mut = "pub")]
  error_cache: ErrorCache<'src>,
  file_bump: &'src Arena,
}

impl<'src> Manager<'src> {
  pub fn new(arena: &'src Arena) -> Self {
    Self {
      sources: HashMap::new(),
      error_cache: ErrorCache::new(),
      file_bump: arena,
    }
  }

  pub fn add_source(&mut self, source: &str, filename: &str) -> Arc<Source<'src>, &'src Arena> {
    let source = Source::new(source, filename, self.file_bump);
    let id = *source.filename();
    let source = Arc::new_in(source, self.file_bump);
    self.sources.insert(id, source.clone());
    self.error_cache.add_file(&source);
    source
  }
}

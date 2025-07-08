use std::{
  collections::HashMap,
  sync::Arc,
};

use getset::Getters;
use bumpalo::Bump;

use crate::{
  FileId,
  source::Source,
}; 

#[derive(Getters)]
pub struct Manager<'src> {
  #[getset(get = "pub")]
  sources: HashMap<FileId, Arc<Source<'src>>>,
  #[cfg_attr(feature = "serde", serde(skip))]
  file_bump: Box<Bump>,
}

impl<'src> Manager<'src> {
  pub fn new() -> Self {
    Self {
      sources: HashMap::new(),
      file_bump: Box::new(Bump::new()),
    }
  }

  pub fn file_bump(&'src self) -> &'src Bump {
    &self.file_bump
  }

  pub fn add_source(&mut self, source: &str, filename: &str) -> Arc<Source<'src>> {
    let bump: &'src Bump = unsafe { &*(self.file_bump.as_ref() as *const Bump) };
    let source = Source::new(source, filename, bump);
    let id = source.filename().clone();
    let source = Arc::new(source);
    self.sources.insert(id, source.clone());
    source
  }
}

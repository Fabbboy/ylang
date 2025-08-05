use crate::file::{
  FileId,
  source::Source,
};
use ariadne::{
  Cache,
  Source as AriadneSource,
};
use std::collections::HashMap;

pub struct ErrorCache<'src> {
  files: HashMap<FileId<'src>, AriadneSource<FileId<'src>>>,
}

impl<'src> ErrorCache<'src> {
  pub fn new() -> Self {
    Self {
      files: HashMap::new(),
    }
  }

  pub fn add_file(&mut self, source: &Source<'src>) {
    self
      .files
      .insert(*source.filename(), AriadneSource::from(*source.content()));
  }
}

impl<'src> Cache<FileId<'src>> for ErrorCache<'src> {
  type Storage = FileId<'src>;

  fn fetch(
    &mut self,
    id: &FileId<'src>,
  ) -> Result<&AriadneSource<Self::Storage>, impl std::fmt::Debug> {
    self
      .files
      .get(id)
      .ok_or_else(|| format!("unknown file: {}", id))
  }

  fn display<'a>(&self, id: &'a FileId<'src>) -> Option<impl std::fmt::Display + 'a> {
    Some(id)
  }
}

impl<'src> Default for ErrorCache<'src> {
  fn default() -> Self {
    Self::new()
  }
}

use ariadne::{
  Cache,
  Source as AriadneSource,
};
use crate::file::{
  FileId,
  source::Source,
};
use std::collections::HashMap;

pub struct ErrorCache<'ctx> {
  files: HashMap<FileId<'ctx>, AriadneSource<FileId<'ctx>>>,
}

impl<'ctx> ErrorCache<'ctx> {
  pub fn new() -> Self {
    Self {
      files: HashMap::new(),
    }
  }

  pub fn add_file(&mut self, source: &Source<'ctx>) {
    self
      .files
      .insert(*source.filename(), AriadneSource::from(*source.content()));
  }
}

impl<'ctx> Cache<FileId<'ctx>> for ErrorCache<'ctx> {
  type Storage = FileId<'ctx>;

  fn fetch(
    &mut self,
    id: &FileId<'ctx>,
  ) -> Result<&AriadneSource<Self::Storage>, impl std::fmt::Debug> {
    self
      .files
      .get(id)
      .ok_or_else(|| format!("unknown file: {}", id))
  }

  fn display<'a>(&self, id: &'a FileId<'ctx>) -> Option<impl std::fmt::Display + 'a> {
    Some(id)
  }
}

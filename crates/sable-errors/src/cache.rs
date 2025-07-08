use ariadne::{
  Cache,
  Source as AriadneSource,
};
use sable_common::file::{
  FileId,
  source::Source,
};
use std::{
  collections::HashMap,
  sync::Arc,
};

pub struct ErrorCache {
  files: HashMap<FileId, AriadneSource<FileId>>,
}

impl ErrorCache {
  pub fn new() -> Self {
    Self {
      files: HashMap::new(),
    }
  }

  pub fn add_file(&mut self, source: &Source<'_>) {
    self.files.insert(
      source.filename().clone(),
      AriadneSource::from(Arc::<str>::from(*source.content())),
    );
  }
}

impl Cache<FileId> for ErrorCache {
  type Storage = FileId;

  fn fetch(&mut self, id: &FileId) -> Result<&AriadneSource<Self::Storage>, impl std::fmt::Debug> {
    self
      .files
      .get(id)
      .ok_or_else(|| format!("unknown file: {}", id))
  }

  fn display<'a>(&self, id: &'a FileId) -> Option<impl std::fmt::Display + 'a> {
    Some(id)
  }
}

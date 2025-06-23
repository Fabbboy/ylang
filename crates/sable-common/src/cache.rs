use std::{
    collections::HashMap,
    sync::Arc,
};
use ariadne::{Cache, Source};

use crate::{
    source::Source as CommonSource,
    FileId,
};

pub struct AriadneCache {
    files: HashMap<FileId, Source<FileId>>,
}

impl AriadneCache {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn add_file(&mut self, source: &CommonSource<'_>) {
        self.files.insert(
            source.id().clone(),
            Source::from(Arc::<str>::from(*source.content())),
        );
    }
}

impl Cache<FileId> for AriadneCache {
    type Storage = FileId;

    fn fetch(&mut self, id: &FileId) -> Result<&Source<Self::Storage>, impl std::fmt::Debug> {
        self.files
            .get(id)
            .ok_or_else(|| format!("unknown file: {}", id))
    }

    fn display<'a>(&self, id: &'a FileId) -> Option<impl std::fmt::Display + 'a> {
        Some(id)
    }
}



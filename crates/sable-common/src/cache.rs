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
            Arc::from(*source.filename()),
            Source::from(Arc::<str>::from(*source.content())),
        );
    }
}

impl Cache<FileId> for AriadneCache {
    type Storage = FileId;

    fn fetch(&mut self, id: &FileId) -> Result<&Source<Self::Storage>, Box<dyn std::fmt::Debug>> {
        self.files
            .get(id)
            .ok_or_else(|| Box::new(format!("unknown file: {}", id)) as Box<dyn std::fmt::Debug>)
    }

    fn display<'a>(&self, id: &'a FileId) -> Option<impl std::fmt::Display + 'a> {
        Some(id)
    }
}



use std::{
  collections::HashMap,
  sync::Arc,
};

use getset::Getters;

use crate::{
  FileId,
  context::Context,
  source::Source,
};

#[derive(Getters)]
pub struct Manager<'ctx> {
  #[getset(get = "pub")]
  sources: HashMap<FileId, Arc<Source<'ctx>>>,
  ctx: &'ctx Context,
}

impl<'ctx> Manager<'ctx> {
  pub fn new(ctx: &'ctx Context) -> Self {
    Self {
      sources: HashMap::new(),
      ctx,
    }
  }

  pub fn add_source(&mut self, source: &str, filename: &str) -> Arc<Source<'ctx>> {
    let source = Source::new(source, filename, self.ctx);
    let id = source.filename().clone();
    let source = Arc::new(source);
    self.sources.insert(id, source.clone());
    source
  }
}

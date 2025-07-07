use getset::Getters;
use std::sync::Arc;

use crate::{context::Context, FileId};

#[derive(Getters)]
pub struct Source<'ctx> {
  #[getset(get = "pub")]
  content: &'ctx str,
  #[getset(get = "pub")]
  filename: FileId,
}

impl<'ctx> Source<'ctx> {
  pub fn new(content: &str, filename: &str, ctx: &'ctx Context) -> Self {
    Self {
      content: ctx.file_bump().alloc_str(content),
      filename: Arc::from(filename),
    }
  }
}

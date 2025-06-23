use bumpalo::Bump;
use getset::Getters;
use std::sync::Arc;

use crate::FileId;

#[derive(Getters)]
pub struct Source<'ctx> {
  #[getset(get = "pub")]
  content: &'ctx str,
  #[getset(get = "pub")]
  filename: &'ctx str,
  #[getset(get = "pub")]
  id: FileId,
}

impl<'ctx> Source<'ctx> {
  pub fn new(content: &str, filename: &str, bump: &'ctx Bump) -> Self {
    Self {
      content: bump.alloc_str(content),
      filename: bump.alloc_str(filename),
      id: Arc::from(filename),
    }
  }
}

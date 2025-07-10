use getset::Getters;
use sable_arena::arena::Arena;
use std::sync::Arc;

use crate::file::FileId;

#[derive(Getters)]
pub struct Source<'ctx> {
  #[getset(get = "pub")]
  content: &'ctx str,
  #[getset(get = "pub")]
  filename: FileId<'ctx>,
}

impl<'ctx> Source<'ctx> {
  pub fn new(content: &str, filename: &str, arena: &'ctx Arena) -> Self {
    let raw_filename: *const str = filename;
    let arc = unsafe { Arc::from_raw_in(raw_filename, arena) };

    Self {
      content: arena.alloc_str(content),
      filename: FileId(arc),
    }
  }
}

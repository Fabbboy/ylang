use getset::Getters;
use sable_arena::arena::Arena;

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
    Self {
      content: arena.alloc_str(content),
      filename: arena.alloc_str(filename),
    }
  }
}

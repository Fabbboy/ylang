use getset::Getters;
use sable_arena::TypedArena;

use crate::file::FileId;

#[derive(Getters)]
pub struct Source<'src> {
  #[getset(get = "pub")]
  content: &'src str,
  #[getset(get = "pub")]
  filename: FileId<'src>,
}

impl<'src> Source<'src> {
  pub fn new(content: &str, filename: &str, arena: &'src TypedArena<Source<'src>>) -> Self {
    Self {
      content: arena.alloc_str(content),
      filename: arena.alloc_str(filename),
    }
  }
}

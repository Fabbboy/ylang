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
    let arced = Arc::from(filename);
    let arced_raw_ptr = Arc::into_raw(arced); // is leaking we need to recreate the original again without copying to destruct it properly

    let arc = unsafe { Arc::<str, &'ctx Arena>::from_raw_in(arced_raw_ptr, arena) }; // from_raw expects an Arc not a str but its the only way we can create a arc with custom allocator KEEP IT!!!!
    let _ = unsafe { Arc::from_raw(arced_raw_ptr) };

    Self {
      content: arena.alloc_str(content),
      filename: FileId(arc),
    }
  }
}

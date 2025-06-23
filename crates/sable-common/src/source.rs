use bumpalo::Bump;
use getset::Getters;

#[derive(Getters)]
pub struct Source<'ctx> {
  #[getset(get = "pub")]
  content: &'ctx str,
  #[getset(get = "pub")]
  filename: &'ctx str,
}

impl<'ctx> Source<'ctx> {
  pub fn new(content: &str, filename: &'ctx str, bump: &'ctx Bump) -> Self {
    Self {
      content: bump.alloc_str(content),
      filename: bump.alloc_str(filename),
    }
  }
}

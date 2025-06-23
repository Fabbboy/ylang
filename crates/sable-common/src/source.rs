use bumpalo::Bump;

pub struct Source<'ctx> {
  content: &'ctx str,
  filename: &'ctx str,
}

impl<'ctx> Source<'ctx> {
  pub fn new(content: &str, filename: &'ctx str, bump: &'ctx Bump) -> Self {
    Self {
      content: bump.alloc_str(content),
      filename,
    }
  }

  pub fn content(&self) -> &'ctx str {
    self.content
  }

  pub fn filename(&self) -> &'ctx str {
    self.filename
  }
}

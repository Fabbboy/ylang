use bumpalo::{Bump, collections::String as BumpString};
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
pub struct Source<'ctx> {
  #[getset(get = "pub", set = "pub(crate)")]
  content: BumpString<'ctx>,
  #[getset(get = "pub", set = "pub(crate)")]
  filename: BumpString<'ctx>,
}

impl<'ctx> Source<'ctx> {
  pub fn new(content: &str, filename: &'ctx str, bump: &'ctx Bump) -> Self {
    Self {
      content: BumpString::from_str_in(content, bump),
      filename: BumpString::from_str_in(filename, bump),
    }
  }
}

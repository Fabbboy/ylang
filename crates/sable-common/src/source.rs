use bumpalo::{Bump, collections::String as BumpString};
use getset::Getters;

#[derive(Getters)]
pub struct Source<'ctx> {
  #[getset(get = "pub")]
  content: BumpString<'ctx>,
  #[getset(get = "pub")]
  filename: BumpString<'ctx>,
}

impl<'ctx> Source<'ctx> {
  pub fn new(content: &str, filename: &str, bump: &'ctx Bump) -> Self {
    Self {
      content: BumpString::from_str_in(content, bump),
      filename: BumpString::from_str_in(filename, bump),
    }
  }
}

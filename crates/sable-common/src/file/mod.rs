use std::{
  fmt::Display,
  ops::Range,
  sync::Arc,
};

use sable_arena::arena::Arena;

pub mod manager;
pub mod source;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileId<'ctx>(pub Arc<str, &'ctx Arena>);

#[cfg(feature = "serde")]
impl<'ctx> serde::Serialize for FileId<'ctx> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.0.as_ref())
  }
}
impl<'ctx> AsRef<str> for FileId<'ctx> {
  fn as_ref(&self) -> &str {
    &self.0
  }
}
impl<'ctx> Display for FileId<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

pub type Span<'ctx> = (FileId<'ctx>, Range<usize>);

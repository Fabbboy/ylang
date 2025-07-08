use std::{
  ops::Range,
  sync::Arc,
};

pub mod cache;
pub mod manager;
pub mod source;

pub type FileId = Arc<str>;
pub type Span = (FileId, Range<usize>);

pub mod cache;
pub mod manager;
pub mod source;
pub mod writer;
pub mod file;

use std::{
  ops::Range,
  sync::Arc,
};

pub type FileId = Arc<str>;
pub type FileSpan = (FileId, Range<usize>);

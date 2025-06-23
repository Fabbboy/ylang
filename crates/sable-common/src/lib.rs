pub mod manager;
pub mod source;
pub mod cache;
pub mod writer;

use std::sync::Arc;

pub type FileId = Arc<str>;
pub type FileSpan = (FileId, std::ops::Range<usize>);

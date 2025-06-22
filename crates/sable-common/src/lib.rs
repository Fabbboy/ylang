pub mod source;
pub mod manager;

pub use source::Source;
pub use manager::SourceManager;

use std::ops::Range;

pub type RangeUsize = Range<usize>;

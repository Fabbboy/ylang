use std::ops::Range;

pub mod manager;
pub mod source;

pub type FileId<'ctx> = &'ctx str;
pub type Span<'ctx> = (FileId<'ctx>, Range<usize>);

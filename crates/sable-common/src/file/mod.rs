use std::ops::Range;

pub mod manager;
pub mod source;

pub type FileId<'src> = &'src str;
pub type Span<'src> = (FileId<'src>, Range<usize>);

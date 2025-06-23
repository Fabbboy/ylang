use std::{
  io,
  ops::Range,
};

use smallvec::SmallVec;

pub type ReportCache<'ctx> = (&'ctx str, Range<usize>);
pub type Report<'ctx> = ariadne::Report<'ctx, ReportCache<'ctx>>;

const MAX_INLINE_CACHE: usize = 4;

#[derive(Debug)]
pub enum DiagnosticError<'ctx> {
  IoError(io::Error),
  NotFound(&'ctx str),
}

pub trait Diagnostic {
  fn write<'ctx>(&self) -> Report<'ctx>;
  fn cache<'f>(&self) -> SmallVec<[&'f str; MAX_INLINE_CACHE]>;
}

pub trait DiagnosticSink {
  type Error;

  fn report(&mut self, diagnostic: impl Diagnostic) -> Result<(), Self::Error>;
}

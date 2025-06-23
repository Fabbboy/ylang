use std::{ops::Range, sync::Arc};

use smallvec::SmallVec;

use crate::{
  manager::Manager,
  source::Source,
};

pub type ReportCache<'ctx> = (&'ctx str, Range<usize>);
pub type Report<'ctx> = ariadne::Report<'ctx, ReportCache<'ctx>>;

pub const MAX_INLINE_CACHE: usize = 4;

pub trait Diagnostic<E> {
  fn write<'ctx>(&self) -> Report<'ctx>;
  fn sources<'a>(&self, manager: &Manager<'a>) -> Result<SmallVec<[Arc<Source<'a>>; MAX_INLINE_CACHE]>, E>;
}

pub trait DiagnosticSink {
  type Error;

  fn report(&mut self, diagnostic: impl Diagnostic<Self::Error>) -> Result<(), Self::Error>;
}

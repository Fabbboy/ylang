use ariadne::Report;
use crate::file::Span;
use std::io;

use crate::cache::ErrorCache;

pub trait Sink<'ctx> {
  type Error: std::fmt::Debug;
  fn report(&mut self, report: Report<'_, Span<'ctx>>) -> Result<(), Self::Error>;
}

pub trait Reportable<'ctx> {
  fn report(&self) -> Report<'_, Span<'ctx>>;
}

pub struct ReportWriter<'w, 'ctx, O> {
  cache: &'w mut ErrorCache<'ctx>,
  out: &'w mut O,
}

impl<'w, 'ctx, O> ReportWriter<'w, 'ctx, O>
where
  O: io::Write,
{
  pub fn new(cache: &'w mut ErrorCache<'ctx>, out: &'w mut O) -> Self {
    Self { cache, out }
  }
}

impl<'w, 'ctx, O> Sink<'ctx> for ReportWriter<'w, 'ctx, O>
where
  O: io::Write,
{
  type Error = io::Error;

  fn report(&mut self, report: Report<Span<'ctx>>) -> Result<(), Self::Error> {
    report.write(&mut *self.cache, &mut *self.out)
  }
}

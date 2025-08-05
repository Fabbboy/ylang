use crate::file::Span;
use ariadne::Report;
use std::io;

use crate::cache::ErrorCache;

pub trait Sink<'src> {
  type Error: std::fmt::Debug;
  fn report(&mut self, report: Report<'_, Span<'src>>) -> Result<(), Self::Error>;
}

pub trait Reportable<'src> {
  fn report(&self) -> Report<'_, Span<'src>>;
}

pub struct ReportWriter<'w, 'src, O> {
  cache: &'w mut ErrorCache<'src>,
  out: &'w mut O,
}

impl<'w, 'src, O> ReportWriter<'w, 'src, O>
where
  O: io::Write,
{
  pub fn new(cache: &'w mut ErrorCache<'src>, out: &'w mut O) -> Self {
    Self { cache, out }
  }
}

impl<'w, 'src, O> Sink<'src> for ReportWriter<'w, 'src, O>
where
  O: io::Write,
{
  type Error = io::Error;

  fn report(&mut self, report: Report<Span<'src>>) -> Result<(), Self::Error> {
    report.write(&mut *self.cache, &mut *self.out)
  }
}

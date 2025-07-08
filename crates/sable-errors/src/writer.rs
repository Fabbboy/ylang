use ariadne::Report;
use sable_common::file::Span;
use std::io;

use crate::cache::ErrorCache;

pub trait Sink {
  type Error: std::fmt::Debug;
  fn report(&mut self, report: Report<'_, Span>) -> Result<(), Self::Error>;
}

pub trait Reportable {
  fn report(&self) -> Report<'_, Span>;
}

pub struct ReportWriter<'w, O> {
  cache: &'w mut ErrorCache,
  out: &'w mut O,
}

impl<'w, O> ReportWriter<'w, O>
where
  O: io::Write,
{
  pub fn new(cache: &'w mut ErrorCache, out: &'w mut O) -> Self {
    Self { cache, out }
  }
}

impl<'w, O> Sink for ReportWriter<'w, O>
where
  O: io::Write,
{
  type Error = io::Error;

  fn report(&mut self, report: Report<Span>) -> Result<(), Self::Error> {
    report.write(&mut *self.cache, &mut *self.out)
  }
}

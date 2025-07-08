use ariadne::Report;
use std::io;

use super::{
  FileSpan,
  cache::ErrorCache,
};

pub trait Sink {
  type Error: std::fmt::Debug;
  fn report(&mut self, report: Report<'_, FileSpan>) -> Result<(), Self::Error>;
}

pub trait Reportable {
  fn report(&self) -> Report<'_, FileSpan>;
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

  fn report(&mut self, report: Report<FileSpan>) -> Result<(), Self::Error> {
    report.write(&mut *self.cache, &mut *self.out)
  }
}

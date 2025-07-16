use std::io;

use ariadne::{
  Color,
  Report,
  ReportKind,
};

use crate::{
  cache::ErrorCache,
  diagnostic::{
    Diagnostic,
    Severity,
  },
  sink::Sink,
};
use sable_common::file::Span;

pub struct ConsoleSink<'w, 'ctx, O> {
  cache: &'w mut ErrorCache<'ctx>,
  out: &'w mut O,
}

impl<'w, 'ctx, O> ConsoleSink<'w, 'ctx, O>
where
  O: io::Write,
{
  pub fn new(cache: &'w mut ErrorCache<'ctx>, out: &'w mut O) -> Self {
    Self { cache, out }
  }

  fn build_report(diag: &Diagnostic<'ctx>) -> Report<'static, Span<'ctx>> {
    let kind = match diag.severity {
      Severity::Error => ReportKind::Error,
      Severity::Warning => ReportKind::Warning,
      Severity::Info => ReportKind::Custom("Info", Color::Blue),
      Severity::Hint => ReportKind::Custom("Hint", Color::Cyan),
    };

    let mut builder = Report::build(kind, diag.range.clone()).with_message(diag.message.clone());

    if let Some(code) = &diag.code {
      builder = builder.with_code(code.clone());
    }
    builder.finish()
  }
}

impl<'w, 'ctx, O> Sink<'ctx> for ConsoleSink<'w, 'ctx, O>
where
  O: io::Write,
{
  type Error = io::Error;

  fn emit(&mut self, diagnostic: Diagnostic<'ctx>) -> Result<(), Self::Error> {
    let report = Self::build_report(&diagnostic);
    report.write(&mut *self.cache, &mut self.out)
  }
}

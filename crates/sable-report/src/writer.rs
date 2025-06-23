use std::io;

use crate::{
  cache::Cache,
  diagnostic::Diagnostic,
  sink::Sink,
};

pub enum WriterError<'ctx> {
  IO(io::Error),
  FileNotFound(&'ctx str),
}

pub struct DiagnosticWriter<'ctx, 'w, O> {
  cache: &'w Cache<'ctx>,
  out: &'w mut O,
}

impl<'ctx, 'w, O> DiagnosticWriter<'ctx, 'w, O>
where
  O: io::Write,
{
  pub fn new(cache: &'w Cache<'ctx>, out: &'w mut O) -> Self {
    Self { cache, out }
  }
}

impl<'ctx, 'w, O> Sink for DiagnosticWriter<'ctx, 'w, O>
where
  O: io::Write,
{
  type Error = WriterError<'ctx>;

  fn report(&mut self, diagnostic: Diagnostic) -> Result<(), Self::Error> {
    Ok(())
  }
}

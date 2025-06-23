use std::io;

use sable_common::manager::Manager;

use crate::sink::Sink;

pub enum WriterError<'ctx> {
  IO(io::Error),
  FileNotFound(&'ctx str),
}

pub struct DiagnosticWriter<'ctx, 'w, O> {
  manager: &'w Manager<'ctx>,
  out: &'w mut O,
}

impl<'ctx, 'w, O> DiagnosticWriter<'ctx, 'w, O>
where
  O: io::Write,
{
  pub fn new(manager: &'w Manager<'ctx>, out: &'w mut O) -> Self {
    Self { manager, out }
  }
}

impl<'ctx, 'w, O> Sink for DiagnosticWriter<'ctx, 'w, O>
where
  O: io::Write,
{
  type Error = WriterError<'ctx>;
}

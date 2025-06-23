use std::io;

use ariadne::sources;
use smallvec::{
  SmallVec,
  smallvec,
};

use crate::{
  manager::Manager,
  report::{
    Diagnostic,
    DiagnosticSink,
  },
};

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

impl<'ctx, 'w, O> DiagnosticSink for DiagnosticWriter<'ctx, 'w, O>
where
  O: io::Write,
{
  type Error = WriterError<'ctx>;

  fn report(&mut self, diagnostic: impl Diagnostic) -> Result<(), Self::Error> {
    let report = diagnostic.write();
    let files = diagnostic.cache();

    let mut srcs: SmallVec<[(&str, &str); 4]> = smallvec![];
    for name in files {
      let source = self
        .manager
        .sources()
        .get(name)
        .ok_or(Self::Error::FileNotFound(name))?;

      srcs.push((name, source.content()));
    }

    report
      .write(sources(srcs), &mut *self.out)
      .map_err(Self::Error::IO)?;

    Ok(())
  }
}

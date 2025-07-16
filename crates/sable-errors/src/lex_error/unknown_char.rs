use sable_common::location::Location;

use crate::{
  diagnostic::{
    Diagnostic,
    Severity,
  },
  sink::Reportable,
};

use sable_common::file::Span;

#[derive(Debug)]
pub struct UnknownCharError<'ctx> {
  pub lexeme: &'ctx str,
  pub location: Location<'ctx>,
}

impl<'ctx> UnknownCharError<'ctx> {
  pub fn new(lexeme: &'ctx str, location: Location<'ctx>) -> Self {
    Self { lexeme, location }
  }

  pub fn report(&self) -> Diagnostic<'ctx> {
    let span: Span<'ctx> = (*self.location.filename(), self.location.range().clone());

    Diagnostic {
      range: span,
      severity: Severity::Error,
      message: format!("Unknown character: `{}`", self.lexeme),
      code: None,
      source: None,
    }
  }
}

impl<'ctx> Reportable<'ctx> for UnknownCharError<'ctx> {
  fn diagnostic(&self) -> Diagnostic<'ctx> {
    self.report()
  }
}

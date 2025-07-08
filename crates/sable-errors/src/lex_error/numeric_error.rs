use ariadne::{
  Label,
  Report,
  ReportKind,
};
use sable_ast::location::Location;
use sable_common::file::Span;

#[derive(Debug)]
pub struct NumericError<'ctx> {
  pub lexeme: &'ctx str,
  pub location: Location,
}

impl<'ctx> NumericError<'ctx> {
  pub fn new(lexeme: &'ctx str, location: Location) -> Self {
    Self { lexeme, location }
  }

  pub fn report(&self) -> ariadne::Report<'_, Span> {
    let span = (
      self.location.filename().clone(),
      self.location.range().clone(),
    );

    Report::build(ReportKind::Error, span.clone())
      .with_message(format!("Invalid number: `{}`", self.lexeme))
      .with_label(Label::new(span).with_message("This number literal is invalid."))
      .finish()
  }
}

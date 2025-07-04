use ariadne::{
  Label,
  Report,
  ReportKind,
};
use sable_ast::location::Location;

#[derive(Debug, Clone)]
pub struct UnknownCharError<'ctx> {
  pub lexeme: &'ctx str,
  pub location: Location,
}

impl<'ctx> UnknownCharError<'ctx> {
  pub fn new(lexeme: &'ctx str, location: Location) -> Self {
    Self { lexeme, location }
  }

  pub fn report(&self) -> ariadne::Report<'_, sable_common::FileSpan> {
    let span = (
      self.location.filename().clone(),
      self.location.range().clone(),
    );

    Report::build(ReportKind::Error, span.clone())
      .with_message(format!("Unknown character: `{}`", self.lexeme))
      .with_label(Label::new(span).with_message("This character is not recognized."))
      .finish()
  }
}

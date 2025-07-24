use ariadne::{
  Label,
  Report,
  ReportKind,
};
use sable_common::{
  file::Span,
  location::Location,
};

#[derive(Debug)]
pub struct NumericError<'ctx> {
  pub lexeme: &'ctx str,
  pub location: Location<'ctx>,
}

impl<'ctx> NumericError<'ctx> {
  pub fn new(lexeme: &'ctx str, location: Location<'ctx>) -> Self {
    Self { lexeme, location }
  }

  pub fn report(&self) -> ariadne::Report<'_, Span<'ctx>> {
    let span = (*self.location.filename(), self.location.range().clone());

    Report::build(ReportKind::Error, span.clone())
      .with_message(format!("Invalid number: `{}`", self.lexeme))
      .with_label(Label::new(span).with_message("This number literal is invalid."))
      .finish()
  }
}

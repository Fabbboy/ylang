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
pub struct UnknownCharError<'ctx> {
  pub lexeme: &'ctx str,
  pub location: Location,
}

impl<'ctx> UnknownCharError<'ctx> {
  pub fn new(lexeme: &'ctx str, location: Location) -> Self {
    Self { lexeme, location }
  }

  pub fn report(&self) -> ariadne::Report<'_, Span> {
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

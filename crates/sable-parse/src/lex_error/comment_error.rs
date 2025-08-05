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
pub struct CommentError<'src> {
  pub lexeme: &'src str,
  pub location: Location<'src>,
}

impl<'src> CommentError<'src> {
  pub fn new(lexeme: &'src str, location: Location<'src>) -> Self {
    Self { lexeme, location }
  }

  pub fn report(&self) -> Report<'_, Span<'src>> {
    let span = (*self.location.filename(), self.location.range().clone());
    Report::build(ReportKind::Error, span.clone())
      .with_message("Unterminated block comment")
      .with_label(Label::new(span).with_message("Comment not terminated"))
      .finish()
  }
}

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
pub struct CommentError<'ctx> {
  pub lexeme: &'ctx str,
  pub location: Location<'ctx>,
}

impl<'ctx> CommentError<'ctx> {
  pub fn new(lexeme: &'ctx str, location: Location<'ctx>) -> Self {
    Self { lexeme, location }
  }

  pub fn report(&self) -> Report<'_, Span<'ctx>> {
    let span = (*self.location.filename(), self.location.range().clone());
    Report::build(ReportKind::Error, span.clone())
      .with_message("Unterminated block comment")
      .with_label(Label::new(span).with_message("Comment not terminated"))
      .finish()
  }
}

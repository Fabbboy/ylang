use ariadne::{
  Label,
  Report,
  ReportKind,
};
use sable_ast::token::{
  Token,
  TokenKind,
};

use sable_common::file::Span;
use smallvec::SmallVec;

use sable_common::writer::Reportable;

pub const MAX_INLINE_KINDS: usize = 8;

#[derive(Debug)]
pub struct UnexpectedTokenError<'src> {
  expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>,
  found: Token<'src>,
}

impl<'src> UnexpectedTokenError<'src> {
  pub fn new(expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>, found: Token<'src>) -> Self {
    Self { expected, found }
  }
}

impl<'src> Reportable<'src> for UnexpectedTokenError<'src> {
  fn report(&self) -> Report<'_, Span<'src>> {
    let span: Span = (
      *self.found.location().filename(),
      self.found.location().range().clone(),
    );

    let expected = format!(
      "Expected one of: {}",
      self
        .expected
        .iter()
        .map(|kind| format!("{:?}", kind))
        .collect::<SmallVec<[String; MAX_INLINE_KINDS]>>()
        .join(", ")
    );

    Report::build(ReportKind::Error, span.clone())
      .with_message(format!("Unexpected token: `{:?}`", self.found.kind()))
      .with_label(Label::new(span).with_message(expected))
      .finish()
  }
}

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

use crate::writer::Reportable;

pub const MAX_INLINE_KINDS: usize = 8;

#[derive(Debug)]
pub struct UnexpectedTokenError<'ctx> {
  expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>,
  found: Token<'ctx>,
}

impl<'ctx> UnexpectedTokenError<'ctx> {
  pub fn new(expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>, found: Token<'ctx>) -> Self {
    Self { expected, found }
  }
}

impl<'ctx> Reportable for UnexpectedTokenError<'ctx> {
  fn report(&self) -> Report<'_, Span> {
    let span: Span = (
      self.found.location().filename().clone(),
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

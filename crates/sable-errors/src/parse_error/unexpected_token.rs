use sable_ast::token::{
  Token,
  TokenKind,
};

use sable_common::file::Span;
use smallvec::SmallVec;

use crate::{
  diagnostic::{
    Diagnostic,
    Severity,
  },
  sink::Reportable,
};

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

impl<'ctx> Reportable<'ctx> for UnexpectedTokenError<'ctx> {
  fn diagnostic(&self) -> Diagnostic<'ctx> {
    let span: Span<'ctx> = (
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

    Diagnostic {
      range: span,
      severity: Severity::Error,
      message: format!("Unexpected token: `{:?}`. {}", self.found.kind(), expected),
      code: None,
      source: None,
    }
  }
}

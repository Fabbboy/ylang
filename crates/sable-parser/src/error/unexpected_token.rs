use sable_report::{
  diagnostic::{
    Diagnostic,
    DiagnosticLevel,
  },
  label::{
    Label,
    LabelKind,
  },
  sink::Report,
  span::Span,
};
use smallvec::SmallVec;

use crate::token::{
  Token,
  TokenKind,
};

pub const MAX_INLINE_KINDS: usize = 8;

#[derive(Debug)]
pub struct UnexpectedToken<'ctx> {
  expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>,
  found: Token<'ctx>,
}

impl<'ctx> UnexpectedToken<'ctx> {
  pub fn new(expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>, found: Token<'ctx>) -> Self {
    Self { expected, found }
  }
}

impl<'ctx> Report for UnexpectedToken<'ctx> {
  type Error = ();

  fn report(&self) -> Diagnostic {
    let code = Span::new(
      self.found.location().range().clone(),
      self.found.location().filename(),
    );

    let expected = format!(
      "Expected one of: {}",
      self
        .expected
        .iter()
        .map(|kind| format!("{:?}", kind))
        .collect::<Vec<_>>()
        .join(", ")
    );

    let label = Label::builder()
      .message(Some(expected))
      .kind(LabelKind::Note)
      .build();

    Diagnostic::builder()
      .message(Some(format!("Unexpected token: `{:?}`", self.found.kind())))
      .code(Some(code))
      .level(DiagnosticLevel::Error)
      .labels(vec![label])
      .build()
  }
}

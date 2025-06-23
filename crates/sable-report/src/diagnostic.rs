use getset::Getters;

use crate::span::Span;

pub enum DiagnosticLevel {
  Error,
  Warning,
  Info,
}

#[derive(Getters)]
pub struct Diagnostic<'ctx> {
  #[getset(get = "pub")]
  level: DiagnosticLevel,
  #[getset(get = "pub")]
  message: Option<&'ctx str>,
  #[getset(get = "pub")]
  code: Option<Span<'ctx>>,
}

impl<'ctx> Diagnostic<'ctx> {
  pub fn new(level: DiagnosticLevel, message: Option<&'ctx str>, code: Option<Span<'ctx>>) -> Self {
    Self {
      level,
      message,
      code,
    }
  }
}

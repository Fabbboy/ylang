use sable_common::file::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
  Error,
  Warning,
  Info,
  Hint,
}

#[derive(Debug, Clone)]
pub struct Diagnostic<'ctx> {
  pub range: Span<'ctx>,
  pub severity: Severity,
  pub message: String,
  pub code: Option<String>,
  pub source: Option<String>,
}

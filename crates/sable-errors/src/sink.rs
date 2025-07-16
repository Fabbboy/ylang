use crate::diagnostic::Diagnostic;

pub trait Sink<'ctx> {
  type Error: std::fmt::Debug;
  fn emit(&mut self, diagnostic: Diagnostic<'ctx>) -> Result<(), Self::Error>;
}

pub trait Reportable<'ctx> {
  fn diagnostic(&self) -> Diagnostic<'ctx>;
}

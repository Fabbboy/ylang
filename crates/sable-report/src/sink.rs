use crate::diagnostic::Diagnostic;

pub trait Sink {
  type Error: std::fmt::Debug;

  fn report(&mut self, diagnostic: Diagnostic) -> Result<(), Self::Error>;
}

pub trait Report {
  type Error;

  fn report(&self) -> Diagnostic;
}

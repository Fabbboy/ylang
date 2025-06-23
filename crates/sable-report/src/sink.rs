use crate::diagnostic::Diagnostic;

pub trait Sink {
  type Error;

  fn report(&mut self, diagnostic: Diagnostic) -> Result<(), Self::Error>;
}

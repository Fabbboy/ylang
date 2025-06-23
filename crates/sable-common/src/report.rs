pub trait Diagnostic<E> {}

pub trait DiagnosticSink {
  type Error;

  fn report(&mut self, diagnostic: impl Diagnostic<Self::Error>) -> Result<(), Self::Error>;
}

use sable_report::{
  diagnostic::Diagnostic,
  sink::Report,
};

pub mod unexpected_token;

#[derive(Debug)]
pub enum ParseError<'ctx> {
  UnexpectedToken(unexpected_token::UnexpectedToken<'ctx>),
}

impl<'ctx> Report for ParseError<'ctx> {
  type Error = ();

  fn report(&self) -> Diagnostic {
    match self {
      ParseError::UnexpectedToken(unexpected_token) => unexpected_token.report(),
    }
  }
}

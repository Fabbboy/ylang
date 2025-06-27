pub mod unexpected_token;

use ariadne::Report;
use sable_common::{
  FileSpan,
  writer::Reportable,
};

#[derive(Debug)]
pub enum ParseError<'ctx> {
  UnexpectedToken(unexpected_token::UnexpectedToken<'ctx>),
}

impl<'ctx> Reportable for ParseError<'ctx> {
  fn report(&self) -> Report<FileSpan> {
    match self {
      ParseError::UnexpectedToken(unexpected_token) => unexpected_token.report(),
    }
  }
}

pub mod unexpected_token;

use ariadne::Report;
use sable_common::{
  FileSpan,
  writer::Reportable,
};

use crate::lex_error::{
    numeric_error::NumericError,
    unknown_char::UnknownCharError,
};

#[derive(Debug)]
pub enum ParseError<'ctx> {
  UnexpectedToken(unexpected_token::UnexpectedTokenError<'ctx>),
  UnknownChar(UnknownCharError<'ctx>),
  NumericError(NumericError<'ctx>),
}

impl<'ctx> Reportable for ParseError<'ctx> {
  fn report(&self) -> Report<FileSpan> {
    match self {
      ParseError::UnexpectedToken(unexpected_token) => unexpected_token.report(),
      ParseError::UnknownChar(unknown_char) => unknown_char.report(),
      ParseError::NumericError(numeric_error) => numeric_error.report(),
    }
  }
}

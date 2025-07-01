pub mod unexpected_token;

use ariadne::Report;
use either::Either;
use sable_common::{
  FileSpan,
  writer::Reportable,
};
use smallvec::SmallVec;

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

pub const MAX_INLINE_ERRORS: usize = 4;

pub struct ParseErrorMOO<'ctx>(
  pub Either<ParseError<'ctx>, SmallVec<[ParseError<'ctx>; MAX_INLINE_ERRORS]>>,
);

impl<'ctx> From<ParseError<'ctx>> for ParseErrorMOO<'ctx> {
  fn from(error: ParseError<'ctx>) -> Self {
    Self(Either::Left(error))
  }
}

pub mod unexpected_token;

use either::Either;
use smallvec::SmallVec;

use crate::{
  diagnostic::Diagnostic,
  lex_error::{
    numeric_error::NumericError,
    unknown_char::UnknownCharError,
  },
  parse_error::unexpected_token::UnexpectedTokenError,
  sink::Reportable,
};

#[derive(Debug)]
pub enum ParseError<'ctx> {
  UnexpectedToken(UnexpectedTokenError<'ctx>),
  UnknownChar(UnknownCharError<'ctx>),
  NumericError(NumericError<'ctx>),
}

impl<'ctx> Reportable<'ctx> for ParseError<'ctx> {
  fn diagnostic(&self) -> Diagnostic<'ctx> {
    match self {
      ParseError::UnexpectedToken(unexpected_token) => unexpected_token.diagnostic(),
      ParseError::UnknownChar(unknown_char) => unknown_char.diagnostic(),
      ParseError::NumericError(numeric_error) => numeric_error.diagnostic(),
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

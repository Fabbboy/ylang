pub mod unexpected_token;

use ariadne::Report;
use either::Either;
use sable_common::file::Span;
use smallvec::SmallVec;

use crate::{
  lex_error::{
    comment_error::CommentError,
    numeric_error::NumericError,
    unknown_char::UnknownCharError,
  },
  parse_error::unexpected_token::UnexpectedTokenError,
};
use sable_common::writer::Reportable;

#[derive(Debug)]
pub enum ParseError<'src> {
  UnexpectedToken(UnexpectedTokenError<'src>),
  UnknownChar(UnknownCharError<'src>),
  NumericError(NumericError<'src>),
  CommentError(CommentError<'src>),
}

impl<'src> Reportable<'src> for ParseError<'src> {
  fn report(&self) -> Report<'_, Span<'src>> {
    match self {
      ParseError::UnexpectedToken(unexpected_token) => unexpected_token.report(),
      ParseError::UnknownChar(unknown_char) => unknown_char.report(),
      ParseError::NumericError(numeric_error) => numeric_error.report(),
      ParseError::CommentError(comment_error) => comment_error.report(),
    }
  }
}

pub const MAX_INLINE_ERRORS: usize = 4;

pub struct ParseErrorMOO<'src>(
  pub Either<ParseError<'src>, SmallVec<[ParseError<'src>; MAX_INLINE_ERRORS]>>,
);

impl<'src> From<ParseError<'src>> for ParseErrorMOO<'src> {
  fn from(error: ParseError<'src>) -> Self {
    Self(Either::Left(error))
  }
}

use either::Either;
use sable_ast::{
  ast::Ast,
  objects::function::Function,
  token::{
    Token,
    TokenError,
    TokenKind,
  },
};
use sable_common::writer::{
  Reportable,
  Sink,
};
use sable_errors::{
  lex_error::{
    numeric_error::NumericError,
    unknown_char::UnknownCharError,
  },
  parse_error::{
    ParseError,
    ParseErrorMOO,
    unexpected_token::{
      MAX_INLINE_KINDS,
      UnexpectedTokenError,
    },
  },
};
use smallvec::{
  SmallVec,
  smallvec,
};

use crate::lexer::Lexer;

pub enum ParseStatus {
  Success,
  Error,
}

pub struct Parser<'ctx, 'p> {
  lexer: Lexer<'ctx>,
  ast: &'p mut Ast,
}

impl<'ctx, 'p> Parser<'ctx, 'p> {
  pub fn new(lexer: Lexer<'ctx>, ast: &'p mut Ast) -> Self {
    Self { lexer, ast }
  }

  fn handle_token_error(&self, token: &Token<'ctx>, error: &TokenError) -> ParseError<'ctx> {
    match error {
      TokenError::UnknownCharacter => ParseError::UnknownChar(UnknownCharError::new(
        token.lexeme(),
        token.location().clone(),
      )),
      TokenError::InvalidInteger | TokenError::InvalidFloat => {
        ParseError::NumericError(NumericError::new(token.lexeme(), token.location().clone()))
      }
    }
  }

  fn handle_parse_error<D>(&self, sink: &mut D, error: ParseErrorMOO<'ctx>)
  where
    D: Sink + ?Sized,
  {
    match error.0 {
      Either::Left(parse_error) => {
        if let Err(e) = sink.report(parse_error.report()) {
          eprintln!("failed to emit diagnostic: {:?}", e);
        }
      }
      Either::Right(errors) => {
        for parse_error in errors {
          if let Err(e) = sink.report(parse_error.report()) {
            eprintln!("failed to emit diagnostic: {:?}", e);
          }
        }
      }
    }
  }

  fn expect(
    &mut self,
    expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>,
  ) -> Result<Token<'ctx>, ParseError<'ctx>> {
    let found = self.lexer.next().unwrap();

    if let TokenKind::Error(token_error) = found.kind() {
      let error = self.handle_token_error(&found, token_error);
      return Err(error);
    }

    if expected.contains(&found.kind().tag()) {
      return Ok(found);
    }
    let unexp = UnexpectedTokenError::new(expected, found);
    Err(ParseError::UnexpectedToken(unexp))
  }

  fn parse_function(&mut self) -> Result<Function, ParseErrorMOO<'ctx>> {
    todo!()
  }

  pub fn parse<D>(&mut self, sink: &mut D) -> ParseStatus
  where
    D: Sink + ?Sized,
  {
    let mut status = ParseStatus::Success;
    let expected = smallvec![TokenKind::Func, TokenKind::Eof];

    loop {
      let kind_tag = match self.expect(expected.clone()) {
        Ok(tok) => tok.kind().tag(),
        Err(error) => {
          self.handle_parse_error(sink, ParseErrorMOO(Either::Left(error)));
          status = ParseStatus::Error;
          continue;
        }
      };

      if kind_tag == TokenKind::Eof {
        break;
      }

      match kind_tag {
        TokenKind::Func => {
          let res = self.parse_function();
          match res {
            Ok(func) => {
              self.ast.funcs_mut().push(func);
            }
            Err(_error) => {
              todo!()
            }
          }
        }
        _ => unreachable!("Unhandled token kind: {:?}", kind_tag),
      }
    }

    status
  }
}

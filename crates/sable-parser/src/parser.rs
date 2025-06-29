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

pub struct Parser<'ctx, 'p, D> {
  lexer: Lexer<'ctx>,
  ast: &'p mut Ast,
  sink: &'p mut D,
}

impl<'ctx, 'p, D> Parser<'ctx, 'p, D>
where
  D: Sink,
{
  pub fn new(lexer: Lexer<'ctx>, ast: &'p mut Ast, reporter: &'p mut D) -> Self {
    Self {
      lexer,
      ast,
      sink: reporter,
    }
  }

  fn token_error(&mut self, token: &Token<'ctx>, error: &TokenError) -> ParseError<'ctx> {
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

  fn expect(
    &mut self,
    expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>,
  ) -> Result<Token<'ctx>, ParseError> {
    let found = self.lexer.next().unwrap();

    if let TokenKind::Error(token_error) = found.kind() {
      let error = self.token_error(&found, token_error);
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

  fn handle_moo(&mut self, error: ParseErrorMOO<'ctx>) {
    match error.0 {
      Either::Left(err) => {
        let report = err.report();
        self.sink.report(report).unwrap();
      }
      _ => unreachable!(),
    }
  }

  pub fn parse(&mut self) -> ParseStatus {
    let mut status = ParseStatus::Success;
    let expected = smallvec![TokenKind::Func, TokenKind::Eof];

    loop {
      let token = self.expect(expected.clone());

      let token = match token {
        Ok(tok) => tok,
        Err(error) => {
          self.handle_moo(error.into());
          status = ParseStatus::Error;
          continue;
        }
      };

      if token.kind().tag() == TokenKind::Eof {
        break;
      }

      match token.kind() {
        TokenKind::Func => {
          let res = self.parse_function();
          match res {
            Ok(func) => {
              let funcs = self.ast.funcs_mut();
              funcs.push(func);
            }
            Err(error) => {
              todo!()
            }
          }
        }
        _ => unreachable!("Unhandled token kind: {:?}", token.kind()),
      }
    }

    status
  }
}

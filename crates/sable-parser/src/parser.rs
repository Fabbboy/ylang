use sable_ast::ast::Ast;
use sable_common::writer::{
  Sink,
  Reportable,
};
use smallvec::{
  SmallVec,
  smallvec,
};

use crate::{
  error::{
    ParseError,
    unexpected_token::{
      MAX_INLINE_KINDS,
      UnexpectedToken,
    },
  },
  lexer::Lexer,
  token::{
    Token,
    TokenKind,
  },
};

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

  fn expect(
    &mut self,
    expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>,
  ) -> Result<Token<'ctx>, ParseError> {
    let found = self.lexer.next().unwrap();

    if expected.contains(&found.kind().tag()) {
      return Ok(found);
    }
    let unexp = UnexpectedToken::new(expected, found);
    Err(ParseError::UnexpectedToken(unexp))
  }

  pub fn parse<D>(&mut self, sink: &mut D) -> ParseStatus
  where
    D: Sink,
  {
    let mut status = ParseStatus::Success;
    let expected = smallvec![TokenKind::Eof];

    loop {
      let token = match self.expect(expected.clone()) {
        Ok(tok) => tok,
        Err(error) => {
          {
            let report = error.report();
            if let Err(e) = sink.report(report) {
              eprintln!("Failed to report diagnostic: {:?}", e);
            }
          }
          status = ParseStatus::Error;
          continue;
        }
      };

      if token.kind().tag() == TokenKind::Eof {
        break;
      }

      match token.kind() {
        _ => unreachable!("Unhandled token kind: {:?}", token.kind()),
      }
    }

    status
  }
}

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
  DiagnosticEngine,
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

pub struct Parser<'ctx, 'p, 'd, S>
where
  S: Sink + ?Sized,
{
  lexer: Lexer<'ctx>,
  ast: &'p mut Ast,
  engine: DiagnosticEngine<'d, S>,
}

impl<'ctx, 'p, 'd, S> Parser<'ctx, 'p, 'd, S>
where
  S: Sink + ?Sized,
{
  pub fn new(lexer: Lexer<'ctx>, ast: &'p mut Ast, sink: &'d mut S) -> Self {
    Self { lexer, ast, engine: DiagnosticEngine::new(sink) }
  }

  fn token_error(token: &Token<'ctx>, error: &TokenError) -> ParseError<'ctx> {
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
    lexer: &mut Lexer<'ctx>,
    expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>,
  ) -> Result<Token<'ctx>, ParseError<'ctx>> {
    let found = lexer.next().unwrap();

    if let TokenKind::Error(token_error) = found.kind() {
      let error = Self::token_error(&found, token_error);
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


  pub fn parse(&mut self) -> ParseStatus {
    let mut status = ParseStatus::Success;
    let expected = smallvec![TokenKind::Func, TokenKind::Eof];

    loop {
      let kind_tag = match Self::expect(&mut self.lexer, expected.clone()) {
        Ok(tok) => tok.kind().tag(),
        Err(error) => {
          if let Err(e) = self.engine.emit(error) {
            eprintln!("failed to emit diagnostic: {:?}", e);
          }
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

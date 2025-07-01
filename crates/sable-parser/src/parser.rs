use std::rc::Rc;

use either::Either;
use sable_ast::{
  ast::Ast,
  location::Location,
  objects::function::Function,
  token::{Token, TokenError, TokenKind},
  types::{PrimitiveType, Type, TypeNamePair},
};
use sable_common::writer::{Reportable, Sink};
use sable_errors::{
  lex_error::{numeric_error::NumericError, unknown_char::UnknownCharError},
  parse_error::{
    ParseError, ParseErrorMOO,
    unexpected_token::{MAX_INLINE_KINDS, UnexpectedTokenError},
  },
};
use smallvec::{SmallVec, smallvec};

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
      Either::Left(parse_error) => sink.report(parse_error.report()).unwrap(),
      Either::Right(errors) => {
        for parse_error in errors {
          sink.report(parse_error.report()).unwrap();
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

  fn sync(&mut self, expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>) {
    loop {
      let next = self.lexer.peek();
      if expected.contains(&next.kind().tag()) {
        return;
      }
      self.lexer.next();
    }
  }

  fn peek(&self, expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>) -> Option<TokenKind> {
    let next = self.lexer.peek();
    if expected.contains(&next.kind().tag()) {
      Some(next.kind().tag())
    } else {
      None
    }
  }

  fn parse_type(&mut self) -> Result<(Type, Location), ParseError<'ctx>> {
    let expected = smallvec![TokenKind::Identifier, TokenKind::Type(PrimitiveType::I32)];
    let token = self.expect(expected)?;

    let start_loc = token.location();

    match token.kind() {
      TokenKind::Identifier => {
        let type_name = Rc::from(*token.lexeme());
        Ok((Type::Custom(type_name), start_loc.clone()))
      }
      TokenKind::Type(primitive_type) => Ok((primitive_type.clone().into(), start_loc.clone())),
      _ => unreachable!("Unhandled token kind: {:?}", token.kind()),
    }
  }

  fn parse_tn_pair(&mut self) -> Result<TypeNamePair, ParseError<'ctx>> {
    let name_token = self.expect(smallvec![TokenKind::Identifier])?;
    let (type_, type_pos) = self.parse_type()?;

    let location = name_token.location().merge(&type_pos).unwrap();
    Ok(
      TypeNamePair::builder()
        .name(Rc::from(*name_token.lexeme()))
        .type_(type_)
        .location(location)
        .build(),
    )
  }

  fn parse_function(&mut self) -> Result<Function, ParseErrorMOO<'ctx>> {
    let func_start_loc = self.expect(smallvec![TokenKind::Func])?.location().clone();

    let name_token = self.expect(smallvec![TokenKind::Identifier])?;
    let func_name = Rc::from(*name_token.lexeme());

    self.expect(smallvec![TokenKind::Paren(true)])?;
    let mut params = SmallVec::new();
    while self.peek(smallvec![TokenKind::Identifier]).is_some() {
      let param = self.parse_tn_pair()?;
      params.push(param.into());
      if self.peek(smallvec![TokenKind::Comma]).is_some() {
        self.expect(smallvec![TokenKind::Comma])?;
      }
    }

    self.expect(smallvec![TokenKind::Paren(false)])?;
    let (return_type, return_type_pos) = self.parse_type()?;

    let signature_pos = func_start_loc.merge(&return_type_pos).unwrap();

    self.expect(smallvec![TokenKind::Brace(true)])?;
    self.expect(smallvec![TokenKind::Brace(false)])?;

    Ok(
      Function::builder()
        .name(func_name)
        .params(params)
        .return_type(return_type)
        .location(signature_pos)
        .build(),
    )
  }

  pub fn parse<D>(&mut self, sink: &mut D) -> ParseStatus
  where
    D: Sink + ?Sized,
  {
    let mut status = ParseStatus::Success;
    let expected = smallvec![TokenKind::Func, TokenKind::Eof];

    loop {
      let kind_tag = match self.peek(expected.clone()) {
        Some(kind) => kind,
        None => {
          let err = self.expect(expected.clone());
          status = ParseStatus::Error;
          match err {
            Ok(_) => unreachable!("Expected error but got a valid token"),
            Err(error) => {
              self.handle_parse_error(sink, error.into());
              self.sync(expected.clone());
              continue;
            }
          }
        }
      };

      if kind_tag == TokenKind::Eof {
        println!("Reached end of file.");
        break;
      }

      match kind_tag {
        TokenKind::Func => {
          let res = self.parse_function();
          match res {
            Ok(func) => {
              self.ast.funcs_mut().push(func);
            }
            Err(error) => {
              self.handle_parse_error(sink, error);
              status = ParseStatus::Error;
              self.sync(expected.clone());
              continue;
            }
          }
        }
        _ => unreachable!("Unhandled token kind: {:?}", kind_tag),
      }
    }

    status
  }
}

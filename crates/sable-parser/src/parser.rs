use std::rc::Rc;

use either::Either;
use sable_ast::{
  ast::Ast,
  expression::{
    Expression,
    block_expression::BlockExpression,
    literal_expression::{
      FloatExpression,
      IntegerExpression,
      LiteralExpression,
    },
  },
  location::Location,
  objects::function::Function,
  statement::Statement,
  token::{
    Token,
    TokenData,
    TokenError,
    TokenKind,
  },
  types::{
    Type,
    TypeNamePair,
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

fn expected_expression() -> SmallVec<[TokenKind; MAX_INLINE_KINDS]> {
  smallvec![TokenKind::Integer, TokenKind::Float,]
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

    if let Some(TokenData::Error(token_error)) = found.data() {
      let error = self.handle_token_error(&found, token_error);
      return Err(error);
    }

    if expected.contains(found.kind()) {
      return Ok(found);
    }
    let unexp = UnexpectedTokenError::new(expected, found);
    Err(ParseError::UnexpectedToken(unexp))
  }

  fn sync(&mut self, expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>) {
    loop {
      let next = self.lexer.peek();
      if expected.contains(&next.kind()) || next.kind().clone() == TokenKind::Eof {
        return;
      }
      self.lexer.next();
    }
  }

  fn peek(&self, expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>) -> Option<TokenKind> {
    let next = self.lexer.peek();
    if expected.contains(next.kind()) {
      Some(*next.kind())
    } else {
      None
    }
  }

  fn parse_type(&mut self) -> Result<(Type, Location), ParseError<'ctx>> {
    let expected = smallvec![TokenKind::Identifier, TokenKind::Type];
    let token = self.expect(expected)?;

    let start_loc = token.location();

    match token.kind() {
      TokenKind::Identifier => {
        let type_name = Rc::from(*token.lexeme());
        Ok((Type::Custom(type_name), start_loc.clone()))
      }
      TokenKind::Type => {
        if let Some(TokenData::Type(primitive_type)) = token.data() {
          Ok((primitive_type.clone().into(), start_loc.clone()))
        } else {
          unreachable!("Type token missing data")
        }
      }
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

  fn parse_factor(&mut self) -> Result<Expression, ParseError<'ctx>> {
    let expected = expected_expression();
    let expr_start = self.expect(expected)?;
    match expr_start.kind() {
      TokenKind::Integer => {
        let value = match expr_start.data() {
          Some(TokenData::Integer(value)) => value,
          _ => unreachable!("Integer token missing data"),
        };

        let int_expr = IntegerExpression::builder()
          .value(*value)
          .location(expr_start.location().clone())
          .build();

        Ok(Expression::Literal(LiteralExpression::Integer(int_expr)))
      }
      TokenKind::Float => {
        let value = match expr_start.data() {
          Some(TokenData::Float(value)) => value,
          _ => unreachable!("Float token missing data"),
        };

        let float_expr = FloatExpression::builder()
          .value(*value)
          .location(expr_start.location().clone())
          .build();

        Ok(Expression::Literal(LiteralExpression::Float(float_expr)))
      }
      _ => unreachable!("Unhandled token kind: {:?}", expr_start.kind()),
    }
  }

  fn parse_term(&mut self) -> Result<Expression, ParseError<'ctx>> {
    let lhs = self.parse_factor()?;
    Ok(lhs)
  }

  fn parse_expression(&mut self) -> Result<Expression, ParseError<'ctx>> {
    let lhs = self.parse_term()?;
    Ok(lhs)
  }

  fn parse_statement(&mut self) -> Result<Statement, ParseErrorMOO<'ctx>> {
    if self.peek(expected_expression()).is_some() {
      let expr = self.parse_expression()?;
      // Try to expect semicolon, but handle missing semicolon gracefully
      if let Err(error) = self.expect(smallvec![TokenKind::Semicolon]) {
        // Convert single parse error to ParseErrorMOO and return it
        return Err(ParseErrorMOO(Either::Left(error)));
      }
      return Ok(Statement::Expression(expr));
    }

    // If we can't parse any recognized statement, return an error instead of panicking
    let next_token = self.lexer.peek();
    let error = ParseError::UnexpectedToken(UnexpectedTokenError::new(
      expected_expression(),
      next_token.clone(),
    ));
    Err(ParseErrorMOO(Either::Left(error)))
  }

  fn parse_block(&mut self) -> Result<BlockExpression, ParseErrorMOO<'ctx>> {
    let mut status = ParseStatus::Success;
    let mut statements = Vec::new();
    let mut errors = SmallVec::new();

    self.expect(smallvec![TokenKind::Brace(true)])?;

    // Sync points for error recovery - we want to continue at semicolons, closing braces, or EOF
    let sync_points = smallvec![
      TokenKind::Semicolon,
      TokenKind::Brace(false),
      TokenKind::Eof
    ];

    while !self
      .peek(smallvec![TokenKind::Brace(false), TokenKind::Eof])
      .is_some()
    {
      match self.parse_statement() {
        Ok(statement) => {
          statements.push(statement);
        }
        Err(error) => {
          match error.0 {
            Either::Left(parse_error) => {
              errors.push(parse_error);
              status = ParseStatus::Error;
            }
            Either::Right(multiple_errors) => {
              errors.extend(multiple_errors);
              status = ParseStatus::Error;
            }
          }

          // Sync to a recovery point
          self.sync(sync_points.clone());

          // If we synchronized to a semicolon, consume it to continue parsing
          if self.peek(smallvec![TokenKind::Semicolon]).is_some() {
            let _ = self.expect(smallvec![TokenKind::Semicolon]);
          }
        }
      }
    }

    // Only expect closing brace if we're not at EOF
    if self.peek(smallvec![TokenKind::Brace(false)]).is_some() {
      self.expect(smallvec![TokenKind::Brace(false)])?;
    }

    match status {
      ParseStatus::Error => Err(ParseErrorMOO(Either::Right(errors))),
      ParseStatus::Success => Ok(
        BlockExpression::builder()
          .body(statements)
          .location(Location::default())
          .build(),
      ),
    }
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

    let mut block = None;
    if self.peek(smallvec![TokenKind::Brace(true)]).is_some() {
      let block_expr = self.parse_block()?;
      block = Some(block_expr);
    }

    Ok(
      Function::builder()
        .name(func_name)
        .params(params)
        .block(block)
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
          if let Err(error) = self.expect(expected.clone()) {
            status = ParseStatus::Error;
            self.handle_parse_error(sink, error.into());
            self.sync(expected.clone());
            continue;
          }
          unreachable!("Expected error but got a valid token")
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

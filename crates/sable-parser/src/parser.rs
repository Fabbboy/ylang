use std::rc::Rc;

use either::Either;
use sable_ast::{
  ast::Ast,
  expression::{
    AssignExpression,
    BinaryExpression,
    BlockExpression,
    Expression,
    IdentifierExpression,
    LiteralExpression,
    binary_expression::{
      AddExpression,
      DivideExpression,
      MultiplyExpression,
      SubtractExpression,
    },
    literal_expression::{
      FloatExpression,
      IntegerExpression,
    },
  },
  location::Location,
  objects::function::Function,
  statement::{
    Statement,
    VariableStatement,
  },
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
  smallvec![TokenKind::Integer, TokenKind::Float, TokenKind::Identifier]
}

pub struct Parser<'ctx, 'p> {
  lexer: Lexer<'ctx>,
  ast: &'p mut Ast<'ctx>,
}

impl<'ctx, 'p> Parser<'ctx, 'p> {
  pub fn new(lexer: Lexer<'ctx>, ast: &'p mut Ast<'ctx>) -> Self {
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
    let found_peek = self.lexer.peek();

    if let Some(TokenData::Error(token_error)) = found_peek.data() {
      let _ = self.lexer.next();
      let error = self.handle_token_error(&found_peek, token_error);
      return Err(error);
    }

    if expected.contains(found_peek.kind()) {
      let found = self.lexer.next().unwrap();
      return Ok(found);
    }

    let unexp = UnexpectedTokenError::new(expected, found_peek);
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

    let mut type_ = match token.kind() {
      TokenKind::Identifier => {
        let type_name = Rc::from(*token.lexeme());
        (Type::Custom(type_name), start_loc.clone())
      }
      TokenKind::Type => {
        if let Some(TokenData::Type(primitive_type)) = token.data() {
          (primitive_type.clone().into(), start_loc.clone())
        } else {
          unreachable!("Type token missing data")
        }
      }
      _ => unreachable!("Unhandled token kind: {:?}", token.kind()),
    };

    while self.peek(smallvec![TokenKind::Star]).is_some() {
      self.expect(smallvec![TokenKind::Star])?;
      type_.0 = Type::Pointer(Box::new(type_.0));
      type_.1 = type_.1.merge(&start_loc).unwrap();
    }

    Ok(type_)
  }

  fn parse_tn_pair(&mut self) -> Result<TypeNamePair, ParseError<'ctx>> {
    let name_token = self.expect(smallvec![TokenKind::Identifier])?;
    self.expect(smallvec![TokenKind::Colon])?;
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

  fn parse_identifier(&mut self) -> Result<Expression, ParseErrorMOO<'ctx>> {
    let identifier = self.expect(smallvec![TokenKind::Identifier])?;
    let maybe_next = match self.peek(smallvec![TokenKind::Assign]) {
      Some(got) => got,
      _ => {
        let id_expr = IdentifierExpression::builder()
          .name(Rc::from(*identifier.lexeme()))
          .location(identifier.location().clone())
          .build();
        return Ok(Expression::Identifier(id_expr));
      }
    };

    match maybe_next {
      TokenKind::Assign => {
        self.expect(smallvec![TokenKind::Assign])?;
        let value = self.parse_expression()?;
        let assign_expr = Expression::Assign(
          AssignExpression::builder()
            .identifier(Rc::from(*identifier.lexeme()))
            .value(Box::new(value))
            .location(identifier.location().clone())
            .build(),
        );
        Ok(assign_expr)
      }
      _ => unreachable!("Expected assignment operator but got: {:?}", maybe_next),
    }
  }

  fn parse_factor(&mut self) -> Result<Expression, ParseErrorMOO<'ctx>> {
    let expected = expected_expression();
    let expr_type = match self.peek(expected.clone()) {
      Some(kind) => kind,
      None => {
        self.expect(expected.clone())?;
        unreachable!("Expected error but got a valid token")
      }
    };

    match expr_type {
      TokenKind::Integer => {
        let value_expr = self.expect(smallvec![TokenKind::Integer])?;

        let value = match value_expr.data() {
          Some(TokenData::Integer(value)) => value,
          _ => unreachable!("Integer token missing data"),
        };

        let int_expr = IntegerExpression::builder()
          .value(*value)
          .location(value_expr.location().clone())
          .build();

        Ok(Expression::Literal(LiteralExpression::Integer(int_expr)))
      }
      TokenKind::Float => {
        let value_expr = self.expect(smallvec![TokenKind::Float])?;

        let value = match value_expr.data() {
          Some(TokenData::Float(value)) => value,
          _ => unreachable!("Float token missing data"),
        };

        let float_expr = FloatExpression::builder()
          .value(*value)
          .location(value_expr.location().clone())
          .build();

        Ok(Expression::Literal(LiteralExpression::Float(float_expr)))
      }
      TokenKind::Identifier => Ok(self.parse_identifier()?),
      _ => unreachable!("Unhandled token kind: {:?}", expr_type),
    }
  }

  fn parse_term(&mut self) -> Result<Expression, ParseErrorMOO<'ctx>> {
    let mut lhs = self.parse_factor()?;

    let expected = smallvec![TokenKind::Star, TokenKind::Slash,];
    while let Some(_) = self.peek(expected.clone()) {
      let op_token = self.expect(expected.clone())?;
      let rhs = self.parse_factor()?;

      let combined_loc = lhs.location().merge(rhs.location()).unwrap();
      match op_token.kind() {
        TokenKind::Star => {
          let expr = MultiplyExpression::builder()
            .left(Box::new(lhs))
            .right(Box::new(rhs))
            .location(combined_loc)
            .build();
          lhs = Expression::Binary(BinaryExpression::Multiply(expr));
        }
        TokenKind::Slash => {
          let expr = DivideExpression::builder()
            .left(Box::new(lhs))
            .right(Box::new(rhs))
            .location(combined_loc)
            .build();
          lhs = Expression::Binary(BinaryExpression::Divide(expr));
        }
        _ => unreachable!("Unhandled token kind: {:?}", op_token.kind()),
      };
    }

    Ok(lhs)
  }

  fn parse_expression(&mut self) -> Result<Expression, ParseErrorMOO<'ctx>> {
    let mut lhs = self.parse_term()?;

    let expected = smallvec![TokenKind::Plus, TokenKind::Minus,];
    if self.peek(expected.clone()).is_some() {
      let op_token = self.expect(expected)?;
      let rhs = self.parse_term()?;

      let combined_loc = lhs.location().merge(rhs.location()).unwrap();
      match op_token.kind() {
        TokenKind::Plus => {
          let expr = AddExpression::builder()
            .left(Box::new(lhs))
            .right(Box::new(rhs))
            .location(combined_loc)
            .build();

          lhs = Expression::Binary(BinaryExpression::Add(expr));
        }
        TokenKind::Minus => {
          let expr = BinaryExpression::Subtract(
            SubtractExpression::builder()
              .left(Box::new(lhs))
              .right(Box::new(rhs))
              .location(combined_loc)
              .build(),
          );
          lhs = Expression::Binary(expr);
        }

        _ => unreachable!("Unhandled token kind: {:?}", op_token.kind()),
      };
    }

    Ok(lhs)
  }

  fn parse_variable_stmt(&mut self) -> Result<VariableStatement, ParseErrorMOO<'ctx>> {
    let var_start = self.expect(smallvec![TokenKind::Var])?;
    let var_name_tok = self.expect(smallvec![TokenKind::Identifier])?;

    let mut type_ = Type::Inference;
    if self.peek(smallvec![TokenKind::Colon]).is_some() {
      self.expect(smallvec![TokenKind::Colon])?;
      let (var_type, _) = self.parse_type()?;
      type_ = var_type;
    }

    self.expect(smallvec![TokenKind::Assign])?;

    let initializer = self.parse_expression()?;
    let var_stop = self.expect(smallvec![TokenKind::Semicolon])?;

    let combined_loc = var_start.location().merge(var_stop.location()).unwrap();

    Ok(
      VariableStatement::builder()
        .name(Rc::from(*var_name_tok.lexeme()))
        .initializer(initializer)
        .type_(type_)
        .location(combined_loc)
        .build(),
    )
  }

  fn parse_statement(&mut self) -> Result<Statement, ParseErrorMOO<'ctx>> {
    if self.peek(expected_expression()).is_some() {
      let expr = self.parse_expression()?;
      self.expect(smallvec![TokenKind::Semicolon])?;

      return Ok(Statement::Expression(expr));
    }

    let expected = smallvec![TokenKind::Var,];

    let stmt_start = match self.peek(expected.clone()) {
      Some(kind) => kind,
      None => {
        if let Err(error) = self.expect(expected.clone()) {
          return Err(ParseErrorMOO(Either::Left(error)));
        }
        unreachable!("Expected error but got a valid token")
      }
    };

    match stmt_start {
      TokenKind::Var => {
        let var_stmt = self.parse_variable_stmt()?;
        return Ok(Statement::Variable(var_stmt));
      }
      _ => unreachable!("Unhandled token kind: {:?}", stmt_start),
    }
  }

  fn parse_block(&mut self) -> Result<BlockExpression, ParseErrorMOO<'ctx>> {
    let mut status = ParseStatus::Success;
    let mut statements = Vec::new();
    let mut errors = SmallVec::new();

    let blk_start = self.expect(smallvec![TokenKind::Brace(true)])?;

    let sync_points = smallvec![TokenKind::Semicolon, TokenKind::Brace(false),];

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

          self.sync(sync_points.clone());

          if self.peek(smallvec![TokenKind::Semicolon]).is_some() {
            let _ = self.expect(smallvec![TokenKind::Semicolon]);
          }
        }
      }
    }

    let blk_end = self.expect(smallvec![TokenKind::Brace(false)])?;
    let merged_loc = blk_start.location().merge(blk_end.location()).unwrap();

    match status {
      ParseStatus::Error => Err(ParseErrorMOO(Either::Right(errors))),
      ParseStatus::Success => Ok(
        BlockExpression::builder()
          .body(statements)
          .location(merged_loc)
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
    self.expect(smallvec![TokenKind::Colon])?;
    let (return_type, return_type_pos) = self.parse_type()?;

    let signature_pos = func_start_loc.merge(&return_type_pos).unwrap();

    let mut block = None;
    if self.peek(smallvec![TokenKind::Brace(true)]).is_some() {
      let block_expr = self.parse_block()?;
      block = Some(block_expr);
    } else {
      self.expect(smallvec![TokenKind::Semicolon])?;
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

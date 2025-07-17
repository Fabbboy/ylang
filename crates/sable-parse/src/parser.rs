use either::Either;
use sable_ast::{
  ast::Ast,
  expression::{
    AssignExpression,
    BinaryExpression,
    BlockExpression,
    Expression,
    ExpressionKind,
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
  located::Located,
  objects::function::Function,
  statement::{
    Statement,
    StatementKind,
    VariableStatement,
  },
  token::{
    Token,
    TokenData,
    TokenError,
    TokenKind,
  },
  types::{
    Path,
    Type,
    TypeNamePair,
  },
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
  writer::{
    Reportable,
    Sink,
  },
};
use smallvec::{
  SmallVec,
  smallvec,
};

use crate::lexer::Lexer;

enum ParseStatus {
  Success,
  Error,
}

fn expected_expression() -> SmallVec<[TokenKind; MAX_INLINE_KINDS]> {
  smallvec![TokenKind::Integer, TokenKind::Float, TokenKind::Identifier]
}

pub struct Parser<'parser, 'ctx, D>
where
  D: Sink<'ctx> + ?Sized,
{
  lexer: Lexer<'ctx>,
  ast: &'parser mut Ast<'ctx>,
  sink: &'parser mut D,
}

impl<'parser, 'ctx, D> Parser<'parser, 'ctx, D>
where
  D: Sink<'ctx> + ?Sized,
{
  pub fn new(lexer: Lexer<'ctx>, ast: &'parser mut Ast<'ctx>, sink: &'parser mut D) -> Self {
    Self { lexer, ast, sink }
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

  fn handle_parse_error(&mut self, error: ParseErrorMOO<'ctx>) {
    match error.0 {
      Either::Left(parse_error) => self.sink.report(parse_error.report()).unwrap(),
      Either::Right(errors) => {
        for parse_error in errors {
          self.sink.report(parse_error.report()).unwrap();
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

  fn peek(&mut self, expected: SmallVec<[TokenKind; MAX_INLINE_KINDS]>) -> Option<TokenKind> {
    let next = self.lexer.peek();
    if expected.contains(next.kind()) {
      Some(*next.kind())
    } else {
      None
    }
  }

  fn parse_type(&mut self) -> Result<Located<'ctx, Type<'ctx>>, ParseError<'ctx>> {
    let token = self.expect(smallvec![TokenKind::Identifier])?;

    let mut ty = Type::Path(Path::builder().segments(vec![token.lexeme()]).build());

    while self.peek(smallvec![TokenKind::Star]).is_some() {
      let _ptr_tok = self.expect(smallvec![TokenKind::Star])?;
      ty = Type::Pointer(Box::new(ty));
    }

    Ok(
      Located::builder()
        .value(ty)
        .location(token.location().clone())
        .build(),
    )
  }

  fn parse_tn_pair(&mut self) -> Result<Located<'ctx, TypeNamePair<'ctx>>, ParseError<'ctx>> {
    let name_token = self.expect(smallvec![TokenKind::Identifier])?;
    self.expect(smallvec![TokenKind::Colon])?;
    let type_located = self.parse_type()?;

    let location = name_token.location().clone();
    Ok(
      Located::builder()
        .value(
          TypeNamePair::builder()
            .name(name_token.lexeme())
            .type_(type_located.value().clone())
            .location(location.clone())
            .build(),
        )
        .location(location)
        .build(),
    )
  }

  fn parse_identifier(&mut self) -> Result<Expression<'ctx>, ParseErrorMOO<'ctx>> {
    let identifier = self.expect(smallvec![TokenKind::Identifier])?;
    let id_located: Located<'ctx, &'ctx str> = identifier.clone().into();

    let maybe_next = match self.peek(smallvec![TokenKind::Assign]) {
      Some(got) => got,
      _ => {
        let id_expr = IdentifierExpression::builder().name(id_located).build();

        return Ok(
          Expression::builder()
            .value(ExpressionKind::Identifier(id_expr))
            .build(),
        );
      }
    };

    match maybe_next {
      TokenKind::Assign => {
        self.expect(smallvec![TokenKind::Assign])?;
        let value = self.parse_expression()?;
        let assign_expr = AssignExpression::builder()
          .identifier(id_located)
          .value(Box::new(value))
          .build();

        Ok(
          Expression::builder()
            .value(ExpressionKind::Assign(assign_expr))
            .build(),
        )
      }
      _ => unreachable!("Expected assignment operator but got: {:?}", maybe_next),
    }
  }

  fn parse_factor(&mut self) -> Result<Expression<'ctx>, ParseErrorMOO<'ctx>> {
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

        let location = Located::builder()
          .value(())
          .location(value_expr.location().clone())
          .build();

        let int_expr = IntegerExpression::builder()
          .value(*value)
          .location(location)
          .build();

        Ok(
          Expression::builder()
            .value(ExpressionKind::Literal(LiteralExpression::Integer(
              int_expr,
            )))
            .build(),
        )
      }
      TokenKind::Float => {
        let value_expr = self.expect(smallvec![TokenKind::Float])?;

        let value = match value_expr.data() {
          Some(TokenData::Float(value)) => value,
          _ => unreachable!("Float token missing data"),
        };

        let location = Located::builder()
          .value(())
          .location(value_expr.location().clone())
          .build();

        let float_expr = FloatExpression::builder()
          .value(*value)
          .location(location)
          .build();
        let expr = ExpressionKind::Literal(LiteralExpression::Float(float_expr));

        Ok(Expression::builder().value(expr).build())
      }
      TokenKind::Identifier => Ok(self.parse_identifier()?),
      _ => unreachable!("Unhandled token kind: {:?}", expr_type),
    }
  }

  fn parse_term(&mut self) -> Result<Expression<'ctx>, ParseErrorMOO<'ctx>> {
    let mut lhs = self.parse_factor()?;

    let expected = smallvec![TokenKind::Star, TokenKind::Slash,];
    while let Some(_) = self.peek(expected.clone()) {
      let op_token = self.expect(expected.clone())?;
      let rhs = self.parse_factor()?;

      match op_token.kind() {
        TokenKind::Star => {
          let expr = MultiplyExpression::builder()
            .left(Box::new(lhs))
            .right(Box::new(rhs))
            .build();
          lhs = Expression::builder()
            .value(ExpressionKind::Binary(BinaryExpression::Multiply(expr)))
            .build();
        }
        TokenKind::Slash => {
          let expr = DivideExpression::builder()
            .left(Box::new(lhs))
            .right(Box::new(rhs))
            .build();

          lhs = Expression::builder()
            .value(ExpressionKind::Binary(BinaryExpression::Divide(expr)))
            .build();
        }
        _ => unreachable!("Unhandled token kind: {:?}", op_token.kind()),
      };
    }

    Ok(lhs)
  }

  fn parse_expression(&mut self) -> Result<Expression<'ctx>, ParseErrorMOO<'ctx>> {
    let mut lhs = self.parse_term()?;

    let expected = smallvec![TokenKind::Plus, TokenKind::Minus,];
    if self.peek(expected.clone()).is_some() {
      let op_token = self.expect(expected)?;
      let rhs = self.parse_term()?;

      match op_token.kind() {
        TokenKind::Plus => {
          let expr = AddExpression::builder()
            .left(Box::new(lhs))
            .right(Box::new(rhs))
            .build();

          lhs = Expression::builder()
            .value(ExpressionKind::Binary(BinaryExpression::Add(expr)))
            .build();
        }
        TokenKind::Minus => {
          let expr = BinaryExpression::Subtract(
            SubtractExpression::builder()
              .left(Box::new(lhs))
              .right(Box::new(rhs))
              .build(),
          );

          lhs = Expression::builder()
            .value(ExpressionKind::Binary(expr))
            .build();
        }

        _ => unreachable!("Unhandled token kind: {:?}", op_token.kind()),
      };
    }

    Ok(lhs)
  }

  fn parse_variable_stmt(&mut self) -> Result<VariableStatement<'ctx>, ParseErrorMOO<'ctx>> {
    let var_start = self.expect(smallvec![TokenKind::Var])?;
    let var_name_tok = self.expect(smallvec![TokenKind::Identifier])?;

    let type_ = if self.peek(smallvec![TokenKind::Colon]).is_some() {
      self.expect(smallvec![TokenKind::Colon])?;
      self.parse_type()?
    } else {
      Located::builder()
        .value(Type::Infer)
        .location(var_start.location().clone())
        .build()
    };

    self.expect(smallvec![TokenKind::Assign])?;

    let initializer = self.parse_expression()?;
    let _var_stop = self.expect(smallvec![TokenKind::Semicolon])?;

    let name_located: Located<'ctx, &'ctx str> = var_name_tok.into();

    Ok(
      VariableStatement::builder()
        .name(name_located)
        .initializer(initializer)
        .type_(type_)
        .build(),
    )
  }

  fn parse_statement(&mut self) -> Result<Statement<'ctx>, ParseErrorMOO<'ctx>> {
    if self.peek(expected_expression()).is_some() {
      let expr = self.parse_expression()?;
      self.expect(smallvec![TokenKind::Semicolon])?;

      return Ok(
        Statement::builder()
          .value(StatementKind::Expression(expr))
          .build(),
      );
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
        return Ok(
          Statement::builder()
            .value(StatementKind::Variable(var_stmt))
            .build(),
        );
      }
      _ => unreachable!("Unhandled token kind: {:?}", stmt_start),
    }
  }

  fn parse_block(&mut self) -> Result<BlockExpression<'ctx>, ParseErrorMOO<'ctx>> {
    let mut status = ParseStatus::Success;
    let mut statements = Vec::new();
    let mut errors = SmallVec::new();

    self.expect(smallvec![TokenKind::Brace(true)])?;

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

    self.expect(smallvec![TokenKind::Brace(false)])?;

    match status {
      ParseStatus::Error => Err(ParseErrorMOO(Either::Right(errors))),
      ParseStatus::Success => Ok(BlockExpression::builder().body(statements).build()),
    }
  }

  fn parse_function(&mut self) -> Result<Function<'ctx>, ParseErrorMOO<'ctx>> {
    let _func_start_loc = self.expect(smallvec![TokenKind::Func])?.location().clone();

    let name_token = self.expect(smallvec![TokenKind::Identifier])?;

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
    let return_type = self.parse_type()?;

    let mut block = None;
    if self.peek(smallvec![TokenKind::Brace(true)]).is_some() {
      let block_expr = self.parse_block()?;
      block = Some(block_expr);
    } else {
      self.expect(smallvec![TokenKind::Semicolon])?;
    }

    let name_located: Located<'ctx, &'ctx str> = name_token.into();

    Ok(
      Function::builder()
        .name(name_located)
        .params(params)
        .block(block)
        .return_type(return_type)
        .build(),
    )
  }

  pub fn parse(&mut self) -> Result<(), ()> {
    self.lexer.reset();

    let mut status = ParseStatus::Success;
    let expected = smallvec![TokenKind::Func, TokenKind::Eof];

    loop {
      let kind_tag = match self.peek(expected.clone()) {
        Some(kind) => kind,
        None => {
          if let Err(error) = self.expect(expected.clone()) {
            status = ParseStatus::Error;
            self.handle_parse_error(error.into());
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
              self.handle_parse_error(error);
              status = ParseStatus::Error;
              self.sync(expected.clone());
              continue;
            }
          }
        }
        _ => unreachable!("Unhandled token kind: {:?}", kind_tag),
      }
    }

    match status {
      ParseStatus::Success => Ok(()),
      ParseStatus::Error => Err(()),
    }
  }
}

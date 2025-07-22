use std::mem::MaybeUninit;

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
  objects::function::{
    Function,
    FunctionParam,
    MAX_INLINE_PARAMS,
  },
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
    Path,
    Type,
    TypeNamePair,
  },
};
use sable_common::location::Location;
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

macro_rules! switch {
  ($expr:expr => {
    $($pattern:pat => $body:expr),* $(,)?
  }) => {
    match $expr {
      $($pattern => $body,)*
      _ => unreachable!("Unhandled case: {:?}", $expr)
    }
  };
}

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

  fn parse_type(&mut self) -> Result<(Type<'ctx>, Location<'ctx>), ParseError<'ctx>> {
    let token = self.expect(smallvec![TokenKind::Identifier])?;

    let segment_located = Located::builder()
      .value(*token.lexeme())
      .location(token.location().clone())
      .build();

    let mut ty = Type::Path(Path::builder().segments(vec![segment_located]).build());

    while self.peek(smallvec![TokenKind::Star]).is_some() {
      let _ptr_tok = self.expect(smallvec![TokenKind::Star])?;
      ty = Type::Pointer(Box::new(ty));
    }

    Ok((ty, token.location().clone()))
  }

  fn parse_tn_pair(&mut self) -> Result<TypeNamePair<'ctx>, ParseError<'ctx>> {
    let name_token = self.expect(smallvec![TokenKind::Identifier])?;
    self.expect(smallvec![TokenKind::Colon])?;
    let (ty, location) = self.parse_type()?;

    let location = name_token.location().merge(&location).unwrap();
    Ok(
      TypeNamePair::builder()
        .name(name_token.lexeme())
        .type_(ty)
        .location(location)
        .build(),
    )
  }

  fn parse_identifier(&mut self) -> Result<Expression<'ctx>, ParseErrorMOO<'ctx>> {
    let identifier = self.expect(smallvec![TokenKind::Identifier])?;

    let maybe_next = match self.peek(smallvec![TokenKind::Assign]) {
      Some(got) => got,
      _ => {
        let name_located = Located::builder()
          .value(*identifier.lexeme())
          .location(identifier.location().clone())
          .build();
        let id_expr = IdentifierExpression::builder()
          .name(name_located)
          .location(identifier.location().clone())
          .build();
        return Ok(
          Expression::builder()
            .value(ExpressionKind::Identifier(id_expr))
            .location(identifier.location().clone())
            .build(),
        );
      }
    };

    switch!(maybe_next => {
      TokenKind::Assign => {
        self.expect(smallvec![TokenKind::Assign])?;
        let value = self.parse_expression()?;
        let value_heaped = self.ast.arena().alloc(value);

        let identifier_located = Located::builder()
          .value(*identifier.lexeme())
          .location(identifier.location().clone())
          .build();
        let assign_expr = AssignExpression::builder()
          .identifier(identifier_located)
          .value(value_heaped)
          .location(identifier.location().clone())
          .build();

        Ok(
          Expression::builder()
            .value(ExpressionKind::Assign(assign_expr))
            .location(identifier.location().clone())
            .build(),
        )
      }
    })
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

    switch!(expr_type => {
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

        Ok(
          Expression::builder()
            .value(ExpressionKind::Literal(LiteralExpression::Integer(
              int_expr,
            )))
            .location(value_expr.location().clone())
            .build(),
        )
      },
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

        Ok(
          Expression::builder()
            .value(ExpressionKind::Literal(LiteralExpression::Float(
              float_expr,
            )))
            .location(value_expr.location().clone())
            .build(),
        )
      },
      TokenKind::Identifier => Ok(self.parse_identifier()?)
    })
  }

  fn parse_term(&mut self) -> Result<Expression<'ctx>, ParseErrorMOO<'ctx>> {
    let mut lhs = self.parse_factor()?;

    let expected = smallvec![TokenKind::Star, TokenKind::Slash,];
    while let Some(_) = self.peek(expected.clone()) {
      let op_token = self.expect(expected.clone())?;
      let rhs = self.parse_factor()?;

      let lhs_loc = lhs.location().clone();
      let rhs_loc = rhs.location().clone();
      let combined = lhs_loc.merge(&rhs_loc).unwrap();

      let lhs_heaped = self.ast.arena().alloc(lhs);
      let rhs_heaped = self.ast.arena().alloc(rhs);

      switch!(op_token.kind() => {
        TokenKind::Star => {
          let expr = MultiplyExpression::builder()
            .left(lhs_heaped)
            .right(rhs_heaped)
            .location(combined.clone())
            .build();
          lhs = Expression::builder()
            .value(ExpressionKind::Binary(BinaryExpression::Multiply(expr)))
            .location(combined.clone())
            .build();
        },
        TokenKind::Slash => {
          let expr = DivideExpression::builder()
            .left(lhs_heaped)
            .right(rhs_heaped)
            .location(combined.clone())
            .build();
          lhs = Expression::builder()
            .value(ExpressionKind::Binary(BinaryExpression::Divide(expr)))
            .location(combined.clone())
            .build();
        }
      });
    }

    Ok(lhs)
  }

  fn parse_expression(&mut self) -> Result<Expression<'ctx>, ParseErrorMOO<'ctx>> {
    let mut lhs = self.parse_term()?;

    let expected = smallvec![TokenKind::Plus, TokenKind::Minus,];
    if self.peek(expected.clone()).is_some() {
      let op_token = self.expect(expected)?;
      let rhs = self.parse_term()?;

      let lhs_loc = lhs.location().clone();
      let rhs_loc = rhs.location().clone();
      let combined = lhs_loc.merge(&rhs_loc).unwrap();

      let lhs_heaped = self.ast.arena().alloc(lhs);
      let rhs_heaped = self.ast.arena().alloc(rhs);

      switch!(op_token.kind() => {
        TokenKind::Plus => {
          let expr = AddExpression::builder()
            .left(lhs_heaped)
            .right(rhs_heaped)
            .location(combined.clone())
            .build();

          lhs = Expression::builder()
            .value(ExpressionKind::Binary(BinaryExpression::Add(expr)))
            .location(combined.clone())
            .build();
        },
        TokenKind::Minus => {
          let expr = BinaryExpression::Subtract(
            SubtractExpression::builder()
              .left(lhs_heaped)
              .right(rhs_heaped)
              .location(combined.clone())
              .build(),
          );

          lhs = Expression::builder()
            .value(ExpressionKind::Binary(expr))
            .location(combined.clone())
            .build();
        }
      });
    }

    Ok(lhs)
  }

  fn parse_variable_stmt(&mut self) -> Result<VariableStatement<'ctx>, ParseErrorMOO<'ctx>> {
    let var_start = self.expect(smallvec![TokenKind::Var])?;
    let var_name_tok = self.expect(smallvec![TokenKind::Identifier])?;

    let type_ = if self.peek(smallvec![TokenKind::Colon]).is_some() {
      self.expect(smallvec![TokenKind::Colon])?;
      let (ty, ty_loc) = self.parse_type()?;
      (ty, ty_loc)
    } else {
      (Type::Infer, var_start.location().clone())
    };

    self.expect(smallvec![TokenKind::Assign])?;

    let initializer = self.parse_expression()?;
    self.expect(smallvec![TokenKind::Semicolon])?;

    let name_located = Located::builder()
      .value(*var_name_tok.lexeme())
      .location(var_name_tok.location().clone())
      .build();

    let type_located = Located::builder().value(type_.0).location(type_.1).build();

    Ok(
      VariableStatement::builder()
        .name(name_located)
        .initializer(initializer)
        .type_(type_located)
        .build(),
    )
  }

  fn parse_statement(&mut self) -> Result<Statement<'ctx>, ParseErrorMOO<'ctx>> {
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

    switch!(stmt_start => {
      TokenKind::Var => {
        let var_stmt = self.parse_variable_stmt()?;
        let stmt_location = var_stmt.name().location().clone();
        let var_stmt_located = Located::builder()
          .value(var_stmt)
          .location(stmt_location)
          .build();
        return Ok(Statement::Variable(var_stmt_located));
      }
    })
  }

  fn parse_block(&mut self) -> Result<BlockExpression<'ctx>, ParseErrorMOO<'ctx>> {
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
    let location = blk_start.location().merge(blk_end.location()).unwrap();

    match status {
      ParseStatus::Error => Err(ParseErrorMOO(Either::Right(errors))),
      ParseStatus::Success => Ok(
        BlockExpression::builder()
          .body(statements)
          .location(location)
          .build(),
      ),
    }
  }

  fn parse_function(&mut self) -> Result<Function<'ctx>, ParseErrorMOO<'ctx>> {
    self.expect(smallvec![TokenKind::Func])?;

    let name_token = self.expect(smallvec![TokenKind::Identifier])?;

    self.expect(smallvec![TokenKind::Paren(true)])?;
    let mut pre_params = SmallVec::<[FunctionParam<'ctx>; MAX_INLINE_PARAMS]>::new();
    while self.peek(smallvec![TokenKind::Identifier]).is_some() {
      let param = self.parse_tn_pair()?;
      let param_location = param.location().clone();
      let param_located = Located::builder()
        .value(param)
        .location(param_location)
        .build();
      pre_params.push(param_located.into());
      if self.peek(smallvec![TokenKind::Comma]).is_some() {
        self.expect(smallvec![TokenKind::Comma])?;
      }
    }

    let raw_params = self
      .ast
      .arena()
      .alloc_slice_with(pre_params.len(), |_| MaybeUninit::uninit());
    for (i, param) in pre_params.into_iter().enumerate() {
      raw_params[i] = MaybeUninit::new(param);
    }
    let final_params: &'ctx [FunctionParam<'ctx>] = unsafe { std::mem::transmute(raw_params) };

    self.expect(smallvec![TokenKind::Paren(false)])?;
    self.expect(smallvec![TokenKind::Colon])?;
    let (return_type, ret_loc) = self.parse_type()?;

    let mut block = None;
    if self.peek(smallvec![TokenKind::Brace(true)]).is_some() {
      let block_expr = self.parse_block()?;
      block = Some(block_expr);
    } else {
      self.expect(smallvec![TokenKind::Semicolon])?;
    }

    let name_located = Located::builder()
      .value(*name_token.lexeme())
      .location(name_token.location().clone())
      .build();

    let return_type_located = Located::builder()
      .value(return_type)
      .location(ret_loc)
      .build();

    Ok(
      Function::builder()
        .name(name_located)
        .params(final_params)
        .block(block)
        .return_type(return_type_located)
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

      switch!(kind_tag => {
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
      })
    }

    match status {
      ParseStatus::Success => Ok(()),
      ParseStatus::Error => Err(()),
    }
  }
}

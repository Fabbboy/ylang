use std::marker::PhantomData;

use sable_ast::{
  ast::Ast,
  expression::{
    AssignExpression,
    BinaryExpression,
    BlockExpression,
    Expression,
    IdentifierExpression,
    LiteralExpression,
    VisitExpression,
  },
  located::Located,
  objects::function::Function,
  statement::{
    VariableStatement,
    VisitStatement,
  },
};
use sable_common::writer::Sink;
use sable_hir::package::Package;

use crate::ast_lower_error::AstLoweringError;

enum ResolverStatus {
  Success,
  OhNo,
}

pub struct Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  asts: &'lower [Ast<'ast>],
  package: &'lower mut Package<'hir>,
  reporter: &'lower mut D,
  _marker: PhantomData<&'src ()>,
}

impl<'src, 'hir, 'ast, 'lower, D> Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  pub fn new(
    asts: &'lower [Ast<'ast>],
    package: &'lower mut Package<'hir>,
    reporter: &'lower mut D,
  ) -> Self {
    Self {
      asts,
      package,
      reporter,
      _marker: PhantomData,
    }
  }

  fn visit_func(&mut self, func: &Function<'ast>) -> Result<(), ()> {
    let status = ResolverStatus::Success;

    match status {
      ResolverStatus::Success => Ok(()),
      ResolverStatus::OhNo => Err(()),
    }
  }

  fn visit_ast(&mut self, ast: &Ast<'ast>) -> Result<(), ()> {
    for func in ast.funcs() {
      self.visit_func(func)?;
    }
    Ok(())
  }

  pub fn resolve(&mut self) -> Result<(), ()> {
    let mut status = ResolverStatus::Success;

    for ast in self.asts.iter() {
      if let Err(_) = self.visit_ast(ast) {
        status = ResolverStatus::OhNo;
      }
    }

    match status {
      ResolverStatus::Success => Ok(()),
      ResolverStatus::OhNo => Err(()),
    }
  }
}

impl<'src, 'hir, 'ast, 'lower, D> VisitExpression<'ast> for Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  type Ret = Result<(), AstLoweringError>;

  fn visit_block(&mut self, block: &Located<'ast, BlockExpression<'ast>>) -> Self::Ret {
    todo!()
  }

  fn visit_literal(&mut self, literal: &Located<'ast, LiteralExpression<'ast>>) -> Self::Ret {
    todo!()
  }

  fn visit_assign(&mut self, assign: &Located<'ast, AssignExpression<'ast>>) -> Self::Ret {
    todo!()
  }

  fn visit_binary(&mut self, binary: &Located<'ast, BinaryExpression<'ast>>) -> Self::Ret {
    todo!()
  }

  fn visit_identifier(
    &mut self,
    identifier: &Located<'ast, IdentifierExpression<'ast>>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_expression(&mut self, expression: &Expression<'ast>) -> Self::Ret {
    match expression {
      Expression::Block(block) => self.visit_block(block),
      Expression::Literal(literal) => self.visit_literal(literal),
      Expression::Assign(assign) => self.visit_assign(assign),
      Expression::Binary(binary) => self.visit_binary(binary),
      Expression::Identifier(identifier) => self.visit_identifier(identifier),
    }
  }
}

impl<'src, 'hir, 'ast, 'lower, D> VisitStatement<'ast> for Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  type Ret = Result<(), AstLoweringError>;

  fn visit_expression(&mut self, expression: &Expression<'ast>) -> Self::Ret {
    todo!()
  }

  fn visit_variable_statement(
    &mut self,
    variable_statement: &Located<'ast, VariableStatement<'ast>>,
  ) -> Self::Ret {
    todo!()
  } // modifys ast and package does not have to return
}

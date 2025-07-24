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
use sable_hir::{
  hir::item::Item,
  package::Package,
};

use crate::ast_lower_error::AstLoweringErrorMMO;

enum LoweringStatus {
  Success,
  OhNo,
}

pub struct AstLowering<'ast, 'hir, 'lower, D>
where
  D: Sink<'hir> + ?Sized,
{
  asts: &'lower [Ast<'ast>],
  package: &'lower Package<'hir>,
  sink: &'lower D,
}

impl<'ast, 'hir, 'lower, D> AstLowering<'ast, 'hir, 'lower, D>
where
  D: Sink<'hir> + ?Sized,
{
  pub fn new(asts: &'lower [Ast<'ast>], package: &'lower Package<'hir>, sink: &'lower D) -> Self {
    Self {
      asts,
      package,
      sink,
    }
  }

  fn lower_func(&mut self, func: &Function<'ast>) -> Result<(), ()> {
    let mut status = LoweringStatus::Success;

    match status {
      LoweringStatus::Success => Ok(()),
      LoweringStatus::OhNo => Err(()),
    }
  }

  fn lower_ast(&mut self, ast: &Ast<'ast>) -> Result<(), ()> {
    let mut status = LoweringStatus::Success;

    for func in ast.funcs() {
      match self.lower_func(func) {
        Ok(()) => {}
        Err(_) => {
          status = LoweringStatus::OhNo;
        }
      }
    }

    match status {
      LoweringStatus::Success => Ok(()),
      LoweringStatus::OhNo => Err(()),
    }
  }

  pub fn lower(&mut self) -> Result<(), ()> {
    let mut status = LoweringStatus::Success;

    for ast in self.asts {
      match self.lower_ast(ast) {
        Ok(()) => {}
        Err(_) => {
          status = LoweringStatus::OhNo;
          break;
        }
      }
    }

    match status {
      LoweringStatus::Success => Ok(()),
      LoweringStatus::OhNo => Err(()),
    }
  }
}

impl<'ast, 'hir, 'lower, D> VisitExpression<'ast> for AstLowering<'ast, 'hir, 'lower, D>
where
  D: Sink<'hir> + ?Sized,
{
  type Ret = Result<Item<'hir>, AstLoweringErrorMMO>;

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
}

impl<'ast, 'lower, 'hir, D> VisitStatement<'ast> for AstLowering<'ast, 'hir, 'lower, D>
where
  D: Sink<'hir> + ?Sized,
{
  type Ret = Result<Item<'hir>, AstLoweringErrorMMO>;

  fn visit_expression(&mut self, expression: &Expression<'ast>) -> Self::Ret {
    todo!()
  }

  fn visit_variable_statement(
    &mut self,
    variable_statement: &Located<'ast, VariableStatement<'ast>>,
  ) -> Self::Ret {
    todo!()
  }
}

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
use sable_common::location::Location;
use sable_hir::package::Package;

enum LoweringStatus {
  Success,
  OhNo,
}

pub struct AstLowering<'ast, 'lower, 'hir> {
  asts: &'lower [Ast<'ast>],
  package: &'lower Package<'hir>,
}

impl<'ast, 'lower, 'hir> AstLowering<'ast, 'lower, 'hir> {
  pub fn new(asts: &'lower [Ast<'ast>], package: &'lower Package<'hir>) -> Self {
    Self { asts, package }
  }

  fn lower_func(&mut self, func: &Function<'ast>) -> Result<(), ()> {
    let mut status = LoweringStatus::Success;

    if let Some(blk) = func.block() {
      match self.visit_block(blk, &blk.location()) {
        Ok(_) => {}
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

impl<'ast, 'lower, 'hir> VisitExpression<'ast> for AstLowering<'ast, 'lower, 'hir> {
  type Result = Result<(), ()>;

  fn visit_block(
    &mut self,
    block: &BlockExpression<'ast>,
    location: &Location<'ast>,
  ) -> Self::Result {
    todo!()
  }

  fn visit_literal(
    &mut self,
    literal: &LiteralExpression,
    location: &Location<'ast>,
  ) -> Self::Result {
    todo!()
  }

  fn visit_assign(
    &mut self,
    assign: &AssignExpression<'ast>,
    location: &Location<'ast>,
  ) -> Self::Result {
    todo!()
  }

  fn visit_binary(
    &mut self,
    binary: &BinaryExpression<'ast>,
    location: &Location<'ast>,
  ) -> Self::Result {
    todo!()
  }

  fn visit_identifier(
    &mut self,
    identifier: &IdentifierExpression<'ast>,
    location: &Location<'ast>,
  ) -> Self::Result {
    todo!()
  }
}

impl<'ast, 'lower, 'hir> VisitStatement<'ast> for AstLowering<'ast, 'lower, 'hir> {
  type Result = Result<(), ()>;

  fn visit_expression(&mut self, expression: &Expression<'ast>) -> Self::Result {
    todo!()
  }

  fn visit_variable_statement(
    &mut self,
    variable_statement: &Located<'ast, VariableStatement<'ast>>,
  ) -> Self::Result {
    todo!()
  }
}

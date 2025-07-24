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
  statement::{
    VariableStatement,
    VisitStatement,
  },
};
use sable_common::location::Location;
use sable_hir::package::Package;

pub struct AstLowering<'ast, 'lower, 'hir> {
  asts: &'lower [Ast<'ast>],
  package: &'lower Package<'hir>,
}

impl<'ast, 'lower, 'hir> AstLowering<'ast, 'lower, 'hir> {
  pub fn new(asts: &'lower [Ast<'ast>], package: &'lower Package<'hir>) -> Self {
    Self { asts, package }
  }

  pub fn lower(&self) -> Result<(), ()> {
    Ok(())
  }
}

impl<'ast, 'lower, 'hir> VisitExpression<'ast> for AstLowering<'ast, 'lower, 'hir> {
  type Result = ();

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
  type Result = ();

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

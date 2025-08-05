use crate::{
  expression::Expression,
  NodeId,
};

pub mod variable_statement;

use getset::{
  Getters,
  MutGetters,
};
use sable_common::{
  location::Location,
  once::Once,
};
use typed_builder::TypedBuilder;
pub use variable_statement::VariableStatement;

#[derive(Debug, TypedBuilder, Getters, MutGetters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Statement<'ast, 'src> {
  #[getset(get = "pub")]
  location: Location<'src>,
  #[getset(get = "pub", get_mut = "pub")]
  kind: StatementKind<'ast, 'src>,
  #[getset(get = "pub", get_mut = "pub")]
  #[builder(default)]
  id: Once<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StatementKind<'ast, 'src> {
  Expression(Expression<'ast, 'src>),
  Variable(VariableStatement<'ast, 'src>),
}

pub trait StatementVisitor<'ast, 'src> {
  type VisitReturn;

  fn visit_expression(
    &mut self,
    expr: &Expression<'ast, 'src>,
    statement: &Statement<'ast, 'src>,
  ) -> Self::VisitReturn;

  fn visit_variable(
    &mut self,
    variable: &VariableStatement<'ast, 'src>,
    statement: &Statement<'ast, 'src>,
  ) -> Self::VisitReturn;

  fn visit_stmt(&mut self, statement: &Statement<'ast, 'src>) -> Self::VisitReturn {
    match &statement.kind {
      StatementKind::Expression(expr) => self.visit_expression(expr, statement),
      StatementKind::Variable(variable) => self.visit_variable(variable, statement),
    }
  }
}

pub trait StatementVisitorMut<'ast, 'src> {
  type VisitReturn;

  fn visit_expression_mut(
    &mut self,
    expr: &mut Expression<'ast, 'src>,
    statement: &mut Statement<'ast, 'src>,
  ) -> Self::VisitReturn;

  fn visit_variable_mut(
    &mut self,
    variable: &mut VariableStatement<'ast, 'src>,
    statement: &mut Statement<'ast, 'src>,
  ) -> Self::VisitReturn;

  fn visit_stmt_mut(&mut self, statement: &mut Statement<'ast, 'src>) -> Self::VisitReturn {
    // SAFETY: see comment in `ExpressionVisitorMut::visit_expr_mut`. We need to
    // borrow both the statement and its inner kind mutably at the same time, so
    // we work with a raw pointer to avoid borrow checker conflicts.
    let stmt_ptr: *mut Statement<'ast, 'src> = statement;
    unsafe {
      match &mut (*stmt_ptr).kind {
        StatementKind::Expression(expr) => self.visit_expression_mut(expr, &mut *stmt_ptr),
        StatementKind::Variable(variable) => self.visit_variable_mut(variable, &mut *stmt_ptr),
      }
    }
  }
}

pub trait VisitableStmt<'ast, 'src> {
  fn accept<V>(&self, statement: &Statement<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: StatementVisitor<'ast, 'src>;
}

pub trait VisitableStmtMut<'ast, 'src> {
  fn accept_mut<V>(&mut self, statement: &mut Statement<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: StatementVisitorMut<'ast, 'src>;
}

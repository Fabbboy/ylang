use crate::{
  NodeId,
  expression::{
    Expression,
    VisitExpression,
    VisitExpressionMut,
  },
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
pub struct Statement<'ctx> {
  #[getset(get = "pub")]
  location: Location<'ctx>,
  #[getset(get = "pub", get_mut = "pub")]
  kind: StatementKind<'ctx>,
  #[getset(get = "pub", get_mut = "pub")]
  #[builder(default)]
  id: Once<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StatementKind<'ctx> {
  Expression(Expression<'ctx>),
  Variable(VariableStatement<'ctx>),
}

pub trait VisitStatement<'ctx>
where
  Self: VisitExpression<'ctx>,
{
  fn visit_variable(
    &mut self,
    statement: &Statement<'ctx>,
    variable_statement: &VariableStatement<'ctx>,
  ) -> Self::Ret;

  fn visit_statement(&mut self, statement: &Statement<'ctx>) -> Self::Ret {
    match statement.kind() {
      StatementKind::Expression(expression) => self.visit_expression(expression),
      StatementKind::Variable(variable_statement) => {
        self.visit_variable(statement, variable_statement)
      }
    }
  }
}

pub trait VisitStatementMut<'ast>
where
  Self: VisitExpressionMut<'ast>,
{
  fn visit_variable(
    &mut self,
    stmt: &mut Statement<'ast>,
    variable_statement: &mut VariableStatement<'ast>,
  ) -> Self::Ret;

  fn visit_statement(&mut self, statement: &mut Statement<'ast>) -> Self::Ret {
    match statement.kind_mut() {
      StatementKind::Expression(expression) => self.visit_expression(expression),
      StatementKind::Variable(variable_statement) => {
        self.visit_variable(statement, variable_statement)
      }
    }
  }
}

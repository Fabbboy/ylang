use crate::{
  NodeId,
  expression::{
    Expression,
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

pub trait VisitStatement<'ctx> {
  type Ret;

  fn visit_expression(
    &mut self,
    id: &Once<NodeId>,
    expression: &Expression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Ret;
  fn visit_variable(
    &mut self,
    id: &Once<NodeId>,
    variable_statement: &VariableStatement<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Ret;

  fn visit_statement(&mut self, statement: &Statement<'ctx>) -> Self::Ret {
    match statement.kind() {
      StatementKind::Expression(expression) => {
        self.visit_expression(statement.id(), expression, statement.location())
      }
      StatementKind::Variable(variable_statement) => {
        self.visit_variable(statement.id(), variable_statement, statement.location())
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
    id: &mut Once<NodeId>,
    variable_statement: &mut VariableStatement<'ast>,
    location: &Location<'ast>,
  ) -> Self::Ret;

  fn visit_statement(&mut self, statement: &mut Statement<'ast>) -> Self::Ret {
    let location = &statement.location;
    let id = &mut statement.id;
    match &mut statement.kind {
      StatementKind::Expression(expression) => self.visit_expression(expression),
      StatementKind::Variable(variable_statement) => {
        self.visit_variable(id, variable_statement, location)
      }
    }
  }
}

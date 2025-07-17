use crate::expression::Expression;
use getset::Getters;
use typed_builder::TypedBuilder;

pub mod variable_statement;

pub use variable_statement::VariableStatement;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Statement<'ctx> {
  #[getset(get = "pub")]
  value: StatementKind<'ctx>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StatementKind<'ctx> {
  Expression(Expression<'ctx>),
  Variable(VariableStatement<'ctx>),
}

pub trait VisitStatement<'ctx> {
  type Result;

  fn visit_expression(&mut self, expression: &Expression<'ctx>) -> Self::Result;
  fn visit_variable_statement(
    &mut self,
    variable_statement: &VariableStatement<'ctx>,
  ) -> Self::Result;

  fn visit_statement(&mut self, statement: &Statement<'ctx>) -> Self::Result {
    match statement.value() {
      StatementKind::Expression(expression) => self.visit_expression(expression),
      StatementKind::Variable(variable_statement) => {
        self.visit_variable_statement(variable_statement)
      }
    }
  }
}

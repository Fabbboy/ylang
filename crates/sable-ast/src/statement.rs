use crate::expression::Expression;

pub mod variable_statement;

pub use variable_statement::VariableStatement;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Statement {
  Expression(Expression),
  Variable(VariableStatement),
}

pub trait VisitStatement {
  type Result;

  fn visit_expression(&mut self, expression: &Expression) -> Self::Result;
  fn visit_variable_statement(&mut self, variable_statement: &VariableStatement) -> Self::Result;

  fn visit_statement(&mut self, statement: &Statement) -> Self::Result {
    match statement {
      Statement::Expression(expression) => self.visit_expression(expression),
      Statement::Variable(variable_statement) => self.visit_variable_statement(variable_statement),
    }
  }
}

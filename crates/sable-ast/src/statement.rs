use crate::expression::Expression;

pub mod variable_statement;

pub use variable_statement::VariableStatement;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Statement<'ctx> {
  Expression(Expression<'ctx>),
  Variable(VariableStatement<'ctx>),
}

pub trait VisitStatement<'ctx> {
  fn visit_expression<T>(&mut self, expression: &Expression<'ctx>) -> T;
  fn visit_variable_statement<T>(
    &mut self,
    variable_statement: &VariableStatement<'ctx>,
  ) -> T;

  fn visit_statement<T>(&mut self, statement: &Statement<'ctx>) -> T {
    match statement {
      Statement::Expression(expression) => self.visit_expression(expression),
      Statement::Variable(variable_statement) => self.visit_variable_statement(variable_statement),
    }
  }
}

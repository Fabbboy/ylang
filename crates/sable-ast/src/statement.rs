use crate::{
  expression::Expression,
  located::Located,
};

pub mod variable_statement;

pub use variable_statement::VariableStatement;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Statement<'ctx> {
  Expression(Expression<'ctx>),
  Variable(Located<'ctx, VariableStatement<'ctx>>),
}

pub trait VisitStatement<'ctx> {
  type Ret;

  fn visit_expression(&mut self, expression: &Expression<'ctx>) -> Self::Ret;
  fn visit_variable_statement(
    &mut self,
    variable_statement: &Located<'ctx, VariableStatement<'ctx>>,
  ) -> Self::Ret;

  fn visit_statement(&mut self, statement: &Statement<'ctx>) -> Self::Ret {
    match statement {
      Statement::Expression(expression) => self.visit_expression(expression),
      Statement::Variable(variable_statement) => self.visit_variable_statement(variable_statement),
    }
  }
}

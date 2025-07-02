use crate::expression::Expression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Statement {
  Expression(Expression),
}

pub trait VisitStatement {
  type Result;

  fn visit_expression(&mut self, expression: &Expression) -> Self::Result;

  fn visit_statement(&mut self, statement: &Statement) -> Self::Result {
    match statement {
      Statement::Expression(expression) => self.visit_expression(expression),
    }
  }
}

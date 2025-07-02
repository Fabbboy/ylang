use crate::expression::{block_expression::BlockExpression, literal_expression::LiteralExpression};

pub mod block_expression;
pub mod literal_expression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Expression {
  Block(BlockExpression),
  Literal(LiteralExpression)
}

pub trait VisitExpression {
  type Result;

  fn visit_block(&mut self, block: &BlockExpression) -> Self::Result;
  fn visit_literal(&mut self, literal: &LiteralExpression) -> Self::Result;

  fn visit_expression(&mut self, expression: &Expression) -> Self::Result {
    match expression {
      Expression::Block(block) => self.visit_block(block),
      Expression::Literal(literal) => self.visit_literal(literal),
    }
  }
}

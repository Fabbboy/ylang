use crate::expression::block_expression::BlockExpression;

pub mod block_expression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Expression {
  Block(BlockExpression),
}

pub trait VisitExpression {
  type Result;

  fn visit_block(&mut self, block: &BlockExpression) -> Self::Result;
  fn visit_expression(&mut self, expression: &Expression) -> Self::Result {
    match expression {
      Expression::Block(block) => self.visit_block(block),
    }
  }
}

pub mod assign_expression;
pub mod binary_expression;
pub mod block_expression;
pub mod literal_expression;

pub use assign_expression::AssignExpression;
pub use binary_expression::BinaryExpression;
pub use block_expression::BlockExpression;
pub use literal_expression::LiteralExpression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expression {
  Block(BlockExpression),
  Literal(LiteralExpression),
  Assign(AssignExpression),
  Binary(BinaryExpression),
}

impl Expression {
  pub fn location(&self) -> &crate::location::Location {
    match self {
      Expression::Block(block) => block.location(),
      Expression::Literal(literal) => literal.location(),
      Expression::Assign(assign) => assign.location(),
      Expression::Binary(binary) => binary.location(),
    }
  }
}

pub trait VisitExpression {
  type Result;

  fn visit_block(&mut self, block: &BlockExpression) -> Self::Result;
  fn visit_literal(&mut self, literal: &LiteralExpression) -> Self::Result;
  fn visit_assign(&mut self, assign: &AssignExpression) -> Self::Result;
  fn visit_binary(&mut self, binary: &BinaryExpression) -> Self::Result;

  fn visit_expression(&mut self, expression: &Expression) -> Self::Result {
    match expression {
      Expression::Block(block) => self.visit_block(block),
      Expression::Literal(literal) => self.visit_literal(literal),
      Expression::Assign(assign) => self.visit_assign(assign),
      Expression::Binary(binary) => self.visit_binary(binary),
    }
  }
}

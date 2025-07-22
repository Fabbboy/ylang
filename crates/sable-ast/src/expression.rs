pub mod assign_expression;
pub mod binary_expression;
pub mod block_expression;
pub mod identifier_expression;
pub mod literal_expression;

pub use assign_expression::AssignExpression;
pub use binary_expression::BinaryExpression;
pub use block_expression::BlockExpression;
pub use identifier_expression::IdentifierExpression;
pub use literal_expression::LiteralExpression;
use sable_common::location::Location;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expression<'ctx> {
  Block(BlockExpression<'ctx>),
  Literal(LiteralExpression<'ctx>),
  Assign(AssignExpression<'ctx>),
  Binary(BinaryExpression<'ctx>),
  Identifier(IdentifierExpression<'ctx>),
}

impl<'ctx> Expression<'ctx> {
  pub fn location(&self) -> &Location<'ctx> {
    match self {
      Expression::Block(block) => block.location(),
      Expression::Literal(literal) => literal.location(),
      Expression::Assign(assign) => assign.location(),
      Expression::Binary(binary) => binary.location(),
      Expression::Identifier(identifier) => identifier.location(),
    }
  }
}

pub trait VisitExpression<'ctx> {
  type Result;

  fn visit_block(&mut self, block: &BlockExpression<'ctx>) -> Self::Result;
  fn visit_literal(&mut self, literal: &LiteralExpression<'ctx>) -> Self::Result;
  fn visit_assign(&mut self, assign: &AssignExpression<'ctx>) -> Self::Result;
  fn visit_binary(&mut self, binary: &BinaryExpression<'ctx>) -> Self::Result;
  fn visit_identifier(&mut self, identifier: &IdentifierExpression<'ctx>) -> Self::Result;

  fn visit_expression(&mut self, expression: &Expression<'ctx>) -> Self::Result {
    match expression {
      Expression::Block(block) => self.visit_block(block),
      Expression::Literal(literal) => self.visit_literal(literal),
      Expression::Assign(assign) => self.visit_assign(assign),
      Expression::Binary(binary) => self.visit_binary(binary),
      Expression::Identifier(identifier) => self.visit_identifier(identifier),
    }
  }
}

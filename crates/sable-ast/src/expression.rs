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

use crate::located::Located;
use sable_common::location::Location;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expression<'ctx> {
  Block(Located<'ctx, BlockExpression<'ctx>>),
  Literal(Located<'ctx, LiteralExpression<'ctx>>),
  Assign(Located<'ctx, AssignExpression<'ctx>>),
  Binary(Located<'ctx, BinaryExpression<'ctx>>),
  Identifier(Located<'ctx, IdentifierExpression<'ctx>>),
}

pub trait VisitExpression<'ctx> {
  type Ret;

  fn visit_block(&mut self, block: &Located<'ctx, BlockExpression<'ctx>>) -> Self::Ret;
  fn visit_literal(&mut self, literal: &Located<'ctx, LiteralExpression<'ctx>>) -> Self::Ret;
  fn visit_assign(&mut self, assign: &Located<'ctx, AssignExpression<'ctx>>) -> Self::Ret;
  fn visit_binary(&mut self, binary: &Located<'ctx, BinaryExpression<'ctx>>) -> Self::Ret;
  fn visit_identifier(
    &mut self,
    identifier: &Located<'ctx, IdentifierExpression<'ctx>>,
  ) -> Self::Ret;

  fn visit_expression(&mut self, expression: &Expression<'ctx>) -> Self::Ret {
    match expression {
      Expression::Block(block) => self.visit_block(block),
      Expression::Literal(literal) => self.visit_literal(literal),
      Expression::Assign(assign) => self.visit_assign(assign),
      Expression::Binary(binary) => self.visit_binary(binary),
      Expression::Identifier(identifier) => self.visit_identifier(identifier),
    }
  }
}

impl<'ctx> Expression<'ctx> {
  pub fn location(&self) -> &Location<'ctx> {
    match self {
      Expression::Block(expr) => expr.location(),
      Expression::Literal(expr) => expr.location(),
      Expression::Assign(expr) => expr.location(),
      Expression::Binary(expr) => expr.location(),
      Expression::Identifier(expr) => expr.location(),
    }
  }
}

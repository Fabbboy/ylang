pub mod assign_expression;
pub mod binary_expression;
pub mod block_expression;
pub mod identifier_expression;
pub mod literal_expression;

pub use assign_expression::AssignExpression;
pub use binary_expression::BinaryExpression;
pub use block_expression::BlockExpression;
use getset::Getters;
pub use identifier_expression::IdentifierExpression;
pub use literal_expression::LiteralExpression;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Expression<'ctx> {
  #[getset(get = "pub")]
  value: ExpressionKind<'ctx>,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ExpressionKind<'ctx> {
  Block(BlockExpression<'ctx>),
  Literal(LiteralExpression<'ctx>),
  Assign(AssignExpression<'ctx>),
  Binary(BinaryExpression<'ctx>),
  Identifier(IdentifierExpression<'ctx>),
}

pub trait VisitExpression<'ctx> {
  type Result;

  fn visit_block(
    &mut self,
    block: &BlockExpression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Result;
  fn visit_literal(
    &mut self,
    literal: &LiteralExpression,
    location: &Location<'ctx>,
  ) -> Self::Result;
  fn visit_assign(
    &mut self,
    assign: &AssignExpression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Result;
  fn visit_binary(
    &mut self,
    binary: &BinaryExpression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Result;
  fn visit_identifier(
    &mut self,
    identifier: &IdentifierExpression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Result;

  fn visit_expression(&mut self, expression: &Expression<'ctx>) -> Self::Result {
    match expression.value() {
      ExpressionKind::Block(block) => self.visit_block(block, expression.location()),
      ExpressionKind::Literal(literal) => self.visit_literal(literal, expression.location()),
      ExpressionKind::Assign(assign) => self.visit_assign(assign, expression.location()),
      ExpressionKind::Binary(binary) => self.visit_binary(binary, expression.location()),
      ExpressionKind::Identifier(identifier) => {
        self.visit_identifier(identifier, expression.location())
      }
    }
  }
}

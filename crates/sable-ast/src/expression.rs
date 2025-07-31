pub mod assign_expression;
pub mod binary_expression;
pub mod block_expression;
pub mod identifier_expression;
pub mod literal_expression;

pub use assign_expression::AssignExpression;
pub use binary_expression::BinaryExpression;
pub use block_expression::BlockExpression;
use getset::{
  Getters,
  MutGetters,
};
pub use identifier_expression::IdentifierExpression;
pub use literal_expression::LiteralExpression;
use typed_builder::TypedBuilder;

use crate::NodeId;
use sable_common::{
  location::Location,
  once::Once,
};

#[derive(Debug, Getters, MutGetters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Expression<'ctx> {
  #[getset(get = "pub")]
  location: Location<'ctx>,
  #[getset(get = "pub")]
  kind: ExpressionKind<'ctx>,
  #[getset(get = "pub", get_mut = "pub")]
  #[builder(default)]
  id: Once<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ExpressionKind<'ctx> {
  Block(BlockExpression<'ctx>),
  Literal(LiteralExpression),
  Assign(AssignExpression<'ctx>),
  Binary(BinaryExpression<'ctx>),
  Identifier(IdentifierExpression<'ctx>),
}

pub trait VisitExpression<'ctx> {
  type Ret;

  fn visit_block(
    &mut self,
    id: &Once<NodeId>,
    block: &BlockExpression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Ret;
  fn visit_literal(
    &mut self,
    id: &Once<NodeId>,
    literal: &LiteralExpression,
    location: &Location<'ctx>,
  ) -> Self::Ret;
  fn visit_assign(
    &mut self,
    id: &Once<NodeId>,
    assign: &AssignExpression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Ret;
  fn visit_binary(
    &mut self,
    id: &Once<NodeId>,
    binary: &BinaryExpression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Ret;
  fn visit_identifier(
    &mut self,
    id: &Once<NodeId>,
    identifier: &IdentifierExpression<'ctx>,
    location: &Location<'ctx>,
  ) -> Self::Ret;

  fn visit_expression(&mut self, expression: &Expression<'ctx>) -> Self::Ret {
    match expression.kind() {
      ExpressionKind::Block(block) => {
        self.visit_block(expression.id(), block, expression.location())
      }
      ExpressionKind::Literal(literal) => {
        self.visit_literal(expression.id(), literal, expression.location())
      }
      ExpressionKind::Assign(assign) => {
        self.visit_assign(expression.id(), assign, expression.location())
      }
      ExpressionKind::Binary(binary) => {
        self.visit_binary(expression.id(), binary, expression.location())
      }
      ExpressionKind::Identifier(identifier) => {
        self.visit_identifier(expression.id(), identifier, expression.location())
      }
    }
  }
}

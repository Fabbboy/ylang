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
pub struct Expression<'ast> {
  #[getset(get = "pub")]
  location: Location<'ast>,
  #[getset(get = "pub", get_mut = "pub")]
  kind: ExpressionKind<'ast>,
  #[getset(get = "pub", get_mut = "pub")]
  #[builder(default)]
  id: Once<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ExpressionKind<'ast> {
  Block(BlockExpression<'ast>),
  Literal(LiteralExpression),
  Assign(AssignExpression<'ast>),
  Binary(BinaryExpression<'ast>),
  Identifier(IdentifierExpression),
}

pub trait ExpressionVisitor<'ast> {
  type VisitReturn;

  fn visit_block(
    &mut self,
    block: &BlockExpression<'ast>,
    expr: &Expression<'ast>,
  ) -> Self::VisitReturn;
  fn visit_literal(
    &mut self,
    literal: &LiteralExpression,
    expr: &Expression<'ast>,
  ) -> Self::VisitReturn;
  fn visit_assign(
    &mut self,
    assign: &AssignExpression<'ast>,
    expr: &Expression<'ast>,
  ) -> Self::VisitReturn;
  fn visit_binary(
    &mut self,
    binary: &BinaryExpression<'ast>,
    expr: &Expression<'ast>,
  ) -> Self::VisitReturn;
  fn visit_identifier(
    &mut self,
    identifier: &IdentifierExpression,
    expr: &Expression<'ast>,
  ) -> Self::VisitReturn;
}

pub trait ExpressionVisitorMut<'ast> {
  type VisitReturn;

  fn visit_block_mut(
    &mut self,
    block: &mut BlockExpression<'ast>,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn;
  fn visit_literal_mut(
    &mut self,
    literal: &mut LiteralExpression,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn;
  fn visit_assign_mut(
    &mut self,
    assign: &mut AssignExpression<'ast>,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn;
  fn visit_binary_mut(
    &mut self,
    binary: &mut BinaryExpression<'ast>,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn;
  fn visit_identifier_mut(
    &mut self,
    identifier: &mut IdentifierExpression,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn;
}

pub trait VisitableExpr<'ast> {
  fn accept<V>(&self, expr: &Expression<'ast>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitor<'ast>;
}

pub trait VisitableExprMut<'ast> {
  fn accept_mut<V>(&mut self, expr: &mut Expression<'ast>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitorMut<'ast>;
}

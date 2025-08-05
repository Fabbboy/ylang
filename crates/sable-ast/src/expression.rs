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
pub struct Expression<'ast, 'src> {
  #[getset(get = "pub")]
  location: Location<'src>,
  #[getset(get = "pub", get_mut = "pub")]
  kind: ExpressionKind<'ast, 'src>,
  #[getset(get = "pub", get_mut = "pub")]
  #[builder(default)]
  id: Once<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ExpressionKind<'ast, 'src> {
  Block(BlockExpression<'ast, 'src>),
  Literal(LiteralExpression),
  Assign(AssignExpression<'ast, 'src>),
  Binary(BinaryExpression<'ast, 'src>),
  Identifier(IdentifierExpression),
}

pub trait ExpressionVisitor<'ast, 'src> {
  type VisitReturn;

  fn visit_block(
    &mut self,
    block: &BlockExpression<'ast, 'src>,
    expr: &Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_literal(
    &mut self,
    literal: &LiteralExpression,
    expr: &Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_assign(
    &mut self,
    assign: &AssignExpression<'ast, 'src>,
    expr: &Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_binary(
    &mut self,
    binary: &BinaryExpression<'ast, 'src>,
    expr: &Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_identifier(
    &mut self,
    identifier: &IdentifierExpression,
    expr: &Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_expr(&mut self, expr: &Expression<'ast, 'src>) -> Self::VisitReturn {
    match &expr.kind {
      ExpressionKind::Block(block) => self.visit_block(block, expr),
      ExpressionKind::Literal(literal) => self.visit_literal(literal, expr),
      ExpressionKind::Assign(assign) => self.visit_assign(assign, expr),
      ExpressionKind::Binary(binary) => self.visit_binary(binary, expr),
      ExpressionKind::Identifier(identifier) => self.visit_identifier(identifier, expr),
    }
  }
}

pub trait ExpressionVisitorMut<'ast, 'src> {
  type VisitReturn;

  fn visit_block_mut(
    &mut self,
    block: &mut BlockExpression<'ast, 'src>,
    expr: &mut Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_literal_mut(
    &mut self,
    literal: &mut LiteralExpression,
    expr: &mut Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_assign_mut(
    &mut self,
    assign: &mut AssignExpression<'ast, 'src>,
    expr: &mut Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_binary_mut(
    &mut self,
    binary: &mut BinaryExpression<'ast, 'src>,
    expr: &mut Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_identifier_mut(
    &mut self,
    identifier: &mut IdentifierExpression,
    expr: &mut Expression<'ast, 'src>,
  ) -> Self::VisitReturn;
  fn visit_expr_mut(&mut self, expr: &mut Expression<'ast, 'src>) -> Self::VisitReturn {
    // SAFETY: We create a raw pointer so that we can obtain simultaneous
    // mutable references to both the expression and its kind. This mirrors
    // the intent of the visitor API while working around the borrow checker,
    // and is safe because the pointer originates from `expr` and is not used
    // after the match statement completes.
    let expr_ptr: *mut Expression<'ast, 'src> = expr;
    unsafe {
      match &mut (*expr_ptr).kind {
        ExpressionKind::Block(block) => self.visit_block_mut(block, &mut *expr_ptr),
        ExpressionKind::Literal(literal) => self.visit_literal_mut(literal, &mut *expr_ptr),
        ExpressionKind::Assign(assign) => self.visit_assign_mut(assign, &mut *expr_ptr),
        ExpressionKind::Binary(binary) => self.visit_binary_mut(binary, &mut *expr_ptr),
        ExpressionKind::Identifier(identifier) => {
          self.visit_identifier_mut(identifier, &mut *expr_ptr)
        }
      }
    }
  }
}

pub trait VisitableExpr<'ast, 'src> {
  fn accept<V>(&self, expr: &Expression<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitor<'ast, 'src>;
}

pub trait VisitableExprMut<'ast, 'src> {
  fn accept_mut<V>(
    &mut self,
    expr: &mut Expression<'ast, 'src>,
    visitor: &mut V,
  ) -> V::VisitReturn
  where
    V: ExpressionVisitorMut<'ast, 'src>;
}

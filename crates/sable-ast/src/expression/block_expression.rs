use crate::{
  expression::{
    Expression,
    ExpressionVisitor,
    ExpressionVisitorMut,
    VisitableExpr,
    VisitableExprMut,
  },
  statement::Statement,
};
use getset::{
  Getters,
  MutGetters,
};
use typed_builder::TypedBuilder;

#[derive(Getters, MutGetters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BlockExpression<'ctx> {
  #[getset(get = "pub", get_mut = "pub")]
  body: Vec<Statement<'ctx>>,
}

impl<'ast> VisitableExpr<'ast> for BlockExpression<'ast> {
  fn accept<V>(&self, expr: &Expression<'ast>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitor<'ast>,
  {
    visitor.visit_block(self, expr)
  }
}

impl<'ast> VisitableExprMut<'ast> for BlockExpression<'ast> {
  fn accept_mut<V>(&mut self, expr: &mut Expression<'ast>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitorMut<'ast>,
  {
    visitor.visit_block_mut(self, expr)
  }
}

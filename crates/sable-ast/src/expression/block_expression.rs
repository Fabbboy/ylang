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
pub struct BlockExpression<'ast, 'src> {
  #[getset(get = "pub", get_mut = "pub")]
  body: Vec<Statement<'ast, 'src>>,
}

impl<'ast, 'src> VisitableExpr<'ast, 'src> for BlockExpression<'ast, 'src> {
  fn accept<V>(&self, expr: &Expression<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitor<'ast, 'src>,
  {
    visitor.visit_block(self, expr)
  }
}

impl<'ast, 'src> VisitableExprMut<'ast, 'src> for BlockExpression<'ast, 'src> {
  fn accept_mut<V>(
    &mut self,
    expr: &mut Expression<'ast, 'src>,
    visitor: &mut V,
  ) -> V::VisitReturn
  where
    V: ExpressionVisitorMut<'ast, 'src>,
  {
    visitor.visit_block_mut(self, expr)
  }
}

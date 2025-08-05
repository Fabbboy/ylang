use getset::{
  Getters,
  MutGetters,
};
use typed_builder::TypedBuilder;

use crate::{
  expression::{
    Expression,
    ExpressionVisitor,
    ExpressionVisitorMut,
    VisitableExpr,
    VisitableExprMut,
  },
  located::Located,
};
use sable_common::interner::Entry;

#[derive(Debug, MutGetters, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssignExpression<'ast, 'src> {
  #[getset(get = "pub")]
  identifier: Located<'src, Entry>,
  #[getset(get = "pub", get_mut = "pub")]
  value: &'ast mut Expression<'ast, 'src>,
}

impl<'ast, 'src> VisitableExpr<'ast, 'src> for AssignExpression<'ast, 'src> {
  fn accept<V>(&self, expr: &Expression<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitor<'ast, 'src>,
  {
    visitor.visit_assign(self, expr)
  }
}

impl<'ast, 'src> VisitableExprMut<'ast, 'src> for AssignExpression<'ast, 'src> {
  fn accept_mut<V>(
    &mut self,
    expr: &mut Expression<'ast, 'src>,
    visitor: &mut V,
  ) -> V::VisitReturn
  where
    V: ExpressionVisitorMut<'ast, 'src>,
  {
    visitor.visit_assign_mut(self, expr)
  }
}

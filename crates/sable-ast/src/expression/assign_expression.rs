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
pub struct AssignExpression<'ctx> {
  #[getset(get = "pub")]
  identifier: Located<'ctx, Entry>,
  #[getset(get = "pub", get_mut = "pub")]
  value: &'ctx mut Expression<'ctx>,
}

impl<'ast> VisitableExpr<'ast> for AssignExpression<'ast> {
  fn accept<V>(&self, expr: &Expression<'ast>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitor<'ast>,
  {
    visitor.visit_assign(self, expr)
  }
}

impl<'ast> VisitableExprMut<'ast> for AssignExpression<'ast> {
  fn accept_mut<V>(&mut self, expr: &mut Expression<'ast>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitorMut<'ast>,
  {
    visitor.visit_assign_mut(self, expr)
  }
}

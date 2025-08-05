use getset::Getters;
use typed_builder::TypedBuilder;

use sable_common::interner::Entry;

use crate::expression::{
  Expression, ExpressionVisitor, ExpressionVisitorMut, VisitableExpr, VisitableExprMut,
};

#[derive(Debug, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IdentifierExpression {
  #[getset(get = "pub")]
  pub name: Entry,
}

impl<'ast, 'src> VisitableExpr<'ast, 'src> for IdentifierExpression {
  fn accept<V>(&self, expr: &Expression<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitor<'ast, 'src>,
  {
    visitor.visit_identifier(self, expr)
  }
}

impl<'ast, 'src> VisitableExprMut<'ast, 'src> for IdentifierExpression {
  fn accept_mut<V>(
    &mut self,
    expr: &mut Expression<'ast, 'src>,
    visitor: &mut V,
  ) -> V::VisitReturn
  where
    V: ExpressionVisitorMut<'ast, 'src>,
  {
    visitor.visit_identifier_mut(self, expr)
  }
}

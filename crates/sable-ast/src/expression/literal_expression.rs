use getset::Getters;
use typed_builder::TypedBuilder;

use crate::expression::{
  Expression,
  ExpressionVisitor,
  ExpressionVisitorMut,
  VisitableExpr,
  VisitableExprMut,
};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum LiteralExpression {
  Integer(IntegerExpression),
  Float(FloatExpression),
}

impl<'ast, 'src> VisitableExpr<'ast, 'src> for LiteralExpression {
  fn accept<V>(&self, expr: &Expression<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: ExpressionVisitor<'ast, 'src>,
  {
    visitor.visit_literal(self, expr)
  }
}

impl<'ast, 'src> VisitableExprMut<'ast, 'src> for LiteralExpression {
  fn accept_mut<V>(
    &mut self,
    expr: &mut Expression<'ast, 'src>,
    visitor: &mut V,
  ) -> V::VisitReturn
  where
    V: ExpressionVisitorMut<'ast, 'src>,
  {
    visitor.visit_literal_mut(self, expr)
  }
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IntegerExpression {
  #[getset(get = "pub")]
  value: i64,
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FloatExpression {
  #[getset(get = "pub")]
  value: f64,
}

use crate::expression::{
  Expression,
  ExpressionVisitor,
  ExpressionVisitorMut,
  VisitableExpr,
  VisitableExprMut,
};
use getset::{
  Getters,
  MutGetters,
};
use typed_builder::TypedBuilder;

macro_rules! binary_expr_factory {
  ($($name:ident),* $(,)?) => {
    paste::paste! {
      $(
        #[derive(Debug, TypedBuilder, Getters, MutGetters)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        pub struct [<$name Expression>]<'ctx> {
          #[getset(get = "pub", get_mut = "pub")]
          left: &'ctx mut Expression<'ctx>,
          #[getset(get = "pub", get_mut = "pub")]
          right: &'ctx mut Expression<'ctx>,
        }
      )*

      #[derive(Debug)]
      #[cfg_attr(feature = "serde", derive(serde::Serialize))]
      pub enum BinaryExpression<'ctx> {
        $(
          $name([<$name Expression>]<'ctx>),
        )*
      }

      impl<'ast> VisitableExpr<'ast> for BinaryExpression<'ast> {
        fn accept<V>(&self, expr: &Expression<'ast>, visitor: &mut V) -> V::VisitReturn
        where
          V: ExpressionVisitor<'ast>,
        {
          visitor.visit_binary(self, expr)
        }
      }

      impl<'ast> VisitableExprMut<'ast> for BinaryExpression<'ast> {
        fn accept_mut<V>(&mut self, expr: &mut Expression<'ast>, visitor: &mut V) -> V::VisitReturn
        where
          V: ExpressionVisitorMut<'ast>,
        {
          visitor.visit_binary_mut(self, expr)
        }
      }
    }
  };
}

binary_expr_factory!(Add, Subtract, Multiply, Divide,);

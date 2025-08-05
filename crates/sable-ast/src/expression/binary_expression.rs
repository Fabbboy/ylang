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
        pub struct [<$name Expression>]<'ast, 'src> {
          #[getset(get = "pub", get_mut = "pub")]
          left: &'ast mut Expression<'ast, 'src>,
          #[getset(get = "pub", get_mut = "pub")]
          right: &'ast mut Expression<'ast, 'src>,
        }
      )*

      #[derive(Debug)]
      #[cfg_attr(feature = "serde", derive(serde::Serialize))]
      pub enum BinaryExpression<'ast, 'src> {
        $(
          $name([<$name Expression>]<'ast, 'src>),
        )*
      }

      impl<'ast, 'src> BinaryExpression<'ast, 'src> {
        pub fn lhs(&self) -> &Expression<'ast, 'src> {
          match self {
            $(
              BinaryExpression::$name(inner) => inner.left(),
            )*
          }
        }

        pub fn lhs_mut(&mut self) -> &mut Expression<'ast, 'src> {
          match self {
            $(
              BinaryExpression::$name(inner) => inner.left_mut(),
            )*
          }
        }

        pub fn rhs(&self) -> &Expression<'ast, 'src> {
          match self {
            $(
              BinaryExpression::$name(inner) => inner.right(),
            )*
          }
        }

        pub fn rhs_mut(&mut self) -> &mut Expression<'ast, 'src> {
          match self {
            $(
              BinaryExpression::$name(inner) => inner.right_mut(),
            )*
          }
        }
      }

      impl<'ast, 'src> VisitableExpr<'ast, 'src> for BinaryExpression<'ast, 'src> {
        fn accept<V>(&self, expr: &Expression<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
        where
          V: ExpressionVisitor<'ast, 'src>,
        {
          visitor.visit_binary(self, expr)
        }
      }

      impl<'ast, 'src> VisitableExprMut<'ast, 'src> for BinaryExpression<'ast, 'src> {
        fn accept_mut<V>(
          &mut self,
          expr: &mut Expression<'ast, 'src>,
          visitor: &mut V,
        ) -> V::VisitReturn
        where
          V: ExpressionVisitorMut<'ast, 'src>,
        {
          visitor.visit_binary_mut(self, expr)
        }
      }
    }
  };
}

binary_expr_factory!(Add, Subtract, Multiply, Divide,);

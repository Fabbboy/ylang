use crate::expression::Expression;
use getset::Getters;
use typed_builder::TypedBuilder;

macro_rules! binary_expr_factory {
  ($($name:ident),* $(,)?) => {
    paste::paste! {
      $(
        #[derive(Debug, TypedBuilder, Getters)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        pub struct [<$name Expression>]<'ctx> {
          #[getset(get = "pub")]
          left: Box<Expression<'ctx>>,
          #[getset(get = "pub")]
          right: Box<Expression<'ctx>>,
        }
      )*

      #[derive(Debug)]
      #[cfg_attr(feature = "serde", derive(serde::Serialize))]
      pub enum BinaryExpression<'ctx> {
        $(
          $name([<$name Expression>]<'ctx>),
        )*
      }
    }
  };
}

binary_expr_factory!(Add, Subtract, Multiply, Divide,);

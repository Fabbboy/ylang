use crate::{
  expression::Expression,
  location::Location,
};
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
          #[getset(get = "pub")]
          location: Location,
        }
      )*

      #[derive(Debug)]
      #[cfg_attr(feature = "serde", derive(serde::Serialize))]
      pub enum BinaryExpression<'ctx> {
        $(
          $name([<$name Expression>]<'ctx>),
        )*
      }

      impl <'ctx>BinaryExpression<'ctx> {
        pub fn location(&self) -> &Location {
          match self {
            $(
              BinaryExpression::$name(expr) => expr.location(),
            )*
          }
        }
      }
    }
  };
}

binary_expr_factory!(Add, Subtract, Multiply, Divide,);

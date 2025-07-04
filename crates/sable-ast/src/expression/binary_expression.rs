use crate::{
  expression::Expression,
  location::Location,
};
use getset::Getters;
use serde::Serialize;
use typed_builder::TypedBuilder;

macro_rules! binary_expr_factory {
  ($($name:ident => $variant:ident),* $(,)?) => {
    paste::paste! {
      $(
        #[derive(Debug, TypedBuilder, Getters)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        pub struct [<$name Expression>] {
          #[getset(get = "pub")]
          left: Box<Expression>,
          #[getset(get = "pub")]
          right: Box<Expression>,
          #[getset(get = "pub")]
          location: Location,
        }
      )*

      #[derive(Debug, Serialize)]
      pub enum BinaryExpression {
        $(
          $name([<$name Expression>]),
        )*
      }

      impl BinaryExpression {
        pub fn location(&self) -> &crate::location::Location {
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

binary_expr_factory!(
  Add => add,
  Subtract => subtract,
  Multiply => multiply,
  Divide => divide,
);

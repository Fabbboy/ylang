use crate::expression::Expression;
use getset::{Getters, MutGetters};
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
    }
  };
}

binary_expr_factory!(Add, Subtract, Multiply, Divide,);

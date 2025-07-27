use getset::{
  Getters,
  MutGetters,
};
use sable_arena::TypedArena;

use crate::{
  expression::Expression,
  objects::function::{
    Function,
    FunctionParam,
  },
};

#[derive(Getters, MutGetters, Debug)]
pub struct Ast<'ctx> {
  #[getset(get_mut = "pub", get = "pub")]
  funcs: Vec<Function<'ctx>>,
  #[getset(get = "pub")]
  expr_arena: &'ctx TypedArena<Expression<'ctx>>,
  #[getset(get = "pub")]
  param_arena: &'ctx TypedArena<FunctionParam<'ctx>>,
}

impl<'ctx> Ast<'ctx> {
  pub fn new(
    expr_arena: &'ctx TypedArena<Expression<'ctx>>,
    param_arena: &'ctx TypedArena<FunctionParam<'ctx>>,
  ) -> Self {
    Ast {
      funcs: Vec::new(),
      expr_arena,
      param_arena,
    }
  }
}

#[cfg(feature = "serde")]
impl<'ctx> serde::Serialize for Ast<'ctx> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeSeq;

    let mut seq = serializer.serialize_seq(Some(self.funcs.len()))?;
    for func in &self.funcs {
      seq.serialize_element(func)?;
    }
    seq.end()
  }
}

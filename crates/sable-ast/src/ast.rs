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
pub struct Ast<'ast, 'src> {
  #[getset(get_mut = "pub", get = "pub")]
  funcs: Vec<Function<'ast, 'src>>,
  #[getset(get = "pub")]
  expr_arena: &'ast TypedArena<Expression<'ast, 'src>>,
  #[getset(get = "pub")]
  param_arena: &'ast TypedArena<FunctionParam<'src>>,
}

impl<'ast, 'src> Ast<'ast, 'src> {
  pub fn new(
    expr_arena: &'ast TypedArena<Expression<'ast, 'src>>,
    param_arena: &'ast TypedArena<FunctionParam<'src>>,
  ) -> Self {
    Ast {
      funcs: Vec::new(),
      expr_arena,
      param_arena,
    }
  }
}

#[cfg(feature = "serde")]
impl<'ast, 'src> serde::Serialize for Ast<'ast, 'src> {
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

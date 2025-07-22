use getset::{
  Getters,
  MutGetters,
};
use sable_arena::arena::Arena;

use crate::objects::function::Function;

#[derive(Getters, MutGetters, Debug)]
pub struct Ast<'ctx> {
  #[getset(get_mut = "pub", get = "pub")]
  funcs: Vec<Function<'ctx>, &'ctx Arena>,
  #[getset(get = "pub")]
  arena: &'ctx Arena,
}

impl<'ctx> Ast<'ctx> {
  pub fn new(arena: &'ctx Arena) -> Self {
    Ast {
      funcs: Vec::new_in(arena),
      arena,
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

use getset::Getters;
use sable_arena::{
  arena::Arena,
  interner::StrInterner,
};

use crate::ty::TypeInterner;

#[derive(Debug, Getters)]
pub struct TyContext<'hir> {
  #[getset(get = "pub")]
  type_map: TypeInterner<'hir>,
  #[getset(get = "pub")]
  str_map: StrInterner<'hir>,
}

impl<'hir> TyContext<'hir> {
  pub fn new(interner_arena: &'hir Arena) -> Self {
    Self {
      type_map: TypeInterner::new(interner_arena),
      str_map: StrInterner::new(interner_arena),
    }
  }
}

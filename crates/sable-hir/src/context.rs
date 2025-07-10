use getset::Getters;
use sable_arena::{
  arena::Arena,
  interner::StrInterner,
};

#[derive(Debug, Getters)]
pub struct TyContext<'hir> {
  #[getset(get = "pub")]
  str_map: StrInterner<'hir>,
}

impl<'hir> TyContext<'hir> {
  pub fn new(interner_arena: &'hir Arena) -> Self {
    Self {
      str_map: StrInterner::new(interner_arena),
    }
  }
}

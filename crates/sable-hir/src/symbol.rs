use std::collections::HashMap;

use bumpalo::Bump;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirSymbol<'arena>(pub &'arena str);

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SymTable<'arena> {
  heap: Bump,
  _marker: std::marker::PhantomData<&'arena ()>,
}

impl<'arena> SymTable<'arena> {
  pub fn new() -> Self {
    Self {
      heap: Bump::new(),
      _marker: std::marker::PhantomData,
    }
  }

  pub fn intern<'f>(&'f self, name: &str) -> HirSymbol<'f> {
    let heaped = self.heap.alloc_str(name);
    HirSymbol(heaped)
  }
}

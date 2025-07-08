use std::{
  collections::HashMap,
  pin::Pin,
};

use bumpalo::Bump;

pub struct StringInterner<'intern> {
  arena: Pin<Box<Bump>>,
  map: HashMap<&'intern str, &'intern str>,
}

impl<'intern> StringInterner<'intern> {
  pub fn new() -> Self {
    Self {
      arena: Box::pin(Bump::new()),
      map: HashMap::new(),
    }
  }

  pub fn intern(&mut self, s: &str) -> &'intern str {
    if let Some(&interned) = self.map.get(s) {
      return interned;
    }

    let interned: &'intern str = unsafe { std::mem::transmute(self.arena.alloc_str(s)) };
    self.map.insert(interned, interned);
    interned
  }
}

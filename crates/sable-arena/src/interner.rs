use std::{
  collections::HashMap,
  hash::Hash,
};

use crate::arena::RawArena;

pub struct Interner<'intern, T, const CHUNK_SIZE: usize = 4096>
where
  T: Eq + Hash + ?Sized,
{
  backing: &'intern RawArena<CHUNK_SIZE>,
  lookup: HashMap<&'intern T, &'intern T>,
}

impl<'intern, T, const CHUNK_SIZE: usize> Interner<'intern, T, CHUNK_SIZE>
where
  T: Eq + Hash + ?Sized + 'intern,
  for<'a> &'a T: Hash,
{
  pub fn new(backing: &'intern RawArena<CHUNK_SIZE>) -> Self {
    Self {
      backing,
      lookup: HashMap::new(),
    }
  }

  pub fn intern(&mut self, value: &'intern T) -> &'intern T {
    if let Some(existing) = self.lookup.get(value) {
      return existing;
    }

    let interned = self.backing.alloc(value).unwrap();
    self.lookup.insert(value, interned);
    interned
  }
}

pub type StrInterner<'intern, const CHUNK_SIZE: usize = 4096> = Interner<'intern, str, CHUNK_SIZE>;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Arena;

  #[test]
  fn test_str_interner() {
    let arena = Arena::new();
    let mut interner = StrInterner::new(&arena);
    let str1 = "hello";
    let str2 = "world";
    let str3 = "hello"; // Duplicate

    let interned1 = interner.intern(str1);
    let interned2 = interner.intern(str2);
    let interned3 = interner.intern(str3);

    assert_eq!(interned1, interned3); // Should be the same
    assert_ne!(interned1, interned2); // Should be different
    assert_eq!(interned1, "hello");
    assert_eq!(interned2, "world");
    assert_eq!(interned3, "hello");
  }
}

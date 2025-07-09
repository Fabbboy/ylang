use std::{
  collections::HashMap,
  hash::Hash,
};

use crate::arena::Arena;

pub struct Interner<'intern, T>
where
  T: Eq + Hash + ?Sized,
{
  backing: &'intern Arena,
  lookup: HashMap<&'intern T, &'intern T>,
}

impl<'intern, T> Interner<'intern, T>
where
  T: Eq + Hash + ?Sized + 'intern,
  for<'a> &'a T: Hash,
{
  pub fn new(backing: &'intern Arena) -> Self {
    Self {
      backing,
      lookup: HashMap::new(),
    }
  }
}

impl<'intern, T> Interner<'intern, T>
where
  T: Eq + Hash + ?Sized,
  for<'a> &'a T: Hash,
{
  pub fn intern(&mut self, value: &T) -> &'intern T {
    if let Some(existing) = self.lookup.get(value) {
      return existing;
    }

    let interned = self.backing.alloc(value);
    let living_interned = unsafe { &*(*interned as *const T) };

    self.lookup.insert(living_interned, living_interned);
    living_interned
  }
}

pub type StrInterner<'intern> = Interner<'intern, str>;

#[cfg(test)]
mod tests {
  use super::*;

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

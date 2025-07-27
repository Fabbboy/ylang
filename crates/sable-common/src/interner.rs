use std::hash::Hash;

use indexmap::IndexSet;
use sable_arena::{
  TypedArena,
  arena::Arena,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entry(pub usize);

pub struct StrInterner<'intern> {
  // SAFETY: we garantee only str's are allocated in this arena
  inner: &'intern Arena,
  indexed: IndexSet<&'intern str>,
}

impl<'intern> StrInterner<'intern> {
  pub fn new(arena: &'intern Arena) -> Self {
    Self {
      inner: arena,
      indexed: IndexSet::new(),
    }
  }

  pub fn intern(&mut self, string: &str) -> Entry {
    if let Some(index) = self.indexed.get_index_of(string) {
      Entry(index)
    } else {
      let copy: &'intern str = self.inner.alloc_str(string);
      let (index, _) = self.indexed.insert_full(copy);
      Entry(index)
    }
  }

  pub fn resolve(&self, symbol: Entry) -> Option<&'intern str> {
    self.indexed.get_index(symbol.0).copied()
  }
}

pub struct Interner<'intern, T>
where
  T: Sized + Eq + Hash,
{
  inner: &'intern TypedArena<T>,
  index: IndexSet<&'intern T>,
}

impl<'intern, T> Interner<'intern, T>
where
  T: Sized + Eq + Hash,
{
  pub fn new(arena: &'intern TypedArena<T>) -> Self {
    Self {
      inner: arena,
      index: IndexSet::new(),
    }
  }

  pub fn intern(&mut self, value: &T) -> Entry {
    if let Some(index) = self.index.get_index_of(value) {
      Entry(index)
    } else {
      let copy: &'intern mut T = self.inner.alloc_copy(value);
      let (index, _) = self.index.insert_full(copy);
      Entry(index)
    }
  }

  pub fn resolve(&self, symbol: Entry) -> Option<&'intern T> {
    self.index.get_index(symbol.0).copied()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  struct Point {
    x: i32,
    y: i32,
  }

  #[test]
  fn test_intern() {
    let arena = TypedArena::<Point>::new();
    let mut interner = Interner::new(&arena);

    let point = Point { x: 1, y: 2 };
    let symbol = interner.intern(&point);
    assert_eq!(interner.resolve(symbol), Some(&point));
  }

  #[test]
  fn test_str_intern() {
    let arena = Arena::new();
    let mut interner = StrInterner::new(&arena);

    let symbol = interner.intern("hello");
    assert_eq!(interner.resolve(symbol), Some("hello"));
  }

  #[test]
  fn test_get_non_existent() {
    let arena = Arena::new();
    let mut interner = StrInterner::new(&arena);

    let symbol = interner.intern("hello");
    assert_eq!(interner.resolve(Entry(symbol.0 + 1)), None);
  }
}

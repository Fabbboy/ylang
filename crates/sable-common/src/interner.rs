use std::{
  cell::RefCell,
  hash::Hash,
};

use heaped::arena::dropless::DroplessArena;
use indexmap::IndexSet;
use sable_arena::{
  TypedArena,
  arena::Arena,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Entry(pub usize);

#[derive(Debug)]
pub struct StrInterner<'intern> {
  // SAFETY: we garantee only str's are allocated in this arena
  inner: &'intern DroplessArena,
  indexed: RefCell<IndexSet<&'intern str>>,
}

impl<'intern> StrInterner<'intern> {
  pub fn new(arena: &'intern DroplessArena) -> Self {
    Self {
      inner: arena,
      indexed: RefCell::new(IndexSet::new()),
    }
  }

  pub fn intern(&self, string: &str) -> Entry {
    let mut set = self.indexed.borrow_mut();
    if let Some(index) = set.get_index_of(string) {
      Entry(index)
    } else {
      let copy = self.inner.alloc_str(string);
      let (index, _) = set.insert_full(copy);
      Entry(index)
    }
  }

  pub fn resolve(&self, symbol: Entry) -> Option<&'intern str> {
    self.indexed.borrow().get_index(symbol.0).copied()
  }
}

pub struct Interner<'intern, T>
where
  T: Sized + Eq + Hash,
{
  inner: &'intern TypedArena<T>,
  index: RefCell<IndexSet<&'intern T>>,
}

impl<'intern, T> Interner<'intern, T>
where
  T: Sized + Eq + Hash,
{
  pub fn new(arena: &'intern TypedArena<T>) -> Self {
    Self {
      inner: arena,
      index: RefCell::new(IndexSet::new()),
    }
  }

  pub fn intern(&self, value: &T) -> Entry {
    let mut index = self.index.borrow_mut();
    if let Some(existing_index) = index.get_index_of(value) {
      Entry(existing_index)
    } else {
      let copy = self.inner.alloc_copy(value);
      let (new_index, _) = index.insert_full(copy);
      Entry(new_index)
    }
  }

  pub fn resolve(&self, symbol: Entry) -> Option<&'intern T> {
    self.index.borrow().get_index(symbol.0).copied()
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
    let interner = Interner::new(&arena);

    let point = Point { x: 1, y: 2 };
    let symbol = interner.intern(&point);
    assert_eq!(interner.resolve(symbol), Some(&point));
  }

  #[test]
  fn test_str_intern() {
    let arena = Arena::new();
    let interner = StrInterner::new(&arena);

    let symbol = interner.intern("hello");
    assert_eq!(interner.resolve(symbol), Some("hello"));
  }

  #[test]
  fn test_get_non_existent() {
    let arena = Arena::new();
    let interner = StrInterner::new(&arena);

    let symbol = interner.intern("hello");
    assert_eq!(interner.resolve(Entry(symbol.0 + 1)), None);
  }
}

use std::{
  array::from_ref,
  hash::Hash,
};

use indexmap::IndexSet;
use sable_arena::TypedArena;

pub struct Entry(pub usize);

// INTERNER COPIES T FULLY INTO THE ARENA 
// IT DOES NOT STORE POINTERS OR REFERENCES
// IT COPIES THE WHOLE DATA
// AND WE DO NOT USE CONTAINERS LIKE VECTORS OR STRINGS INSIDE AN INTERNER
// THIS DEFEATS THE PURPOSE OF INTERNING
pub struct Interner<'intern, T>
where
  T: Hash + Eq + Copy,
{
  arena: &'intern TypedArena<T>,
  strintern: IndexSet<&'intern T>,
}

impl<'intern, T> Interner<'intern, T>
where
  T: Hash + Eq + Copy,
{
  pub fn new(arena: &'intern TypedArena<T>) -> Self {
    Self {
      arena,
      strintern: IndexSet::new(),
    }
  }

  pub fn intern(&mut self, data: &T) -> Entry {
    if self.strintern.contains(data) {
      let idx = self.strintern.get_index_of(data).unwrap();
      Entry(idx)
    } else {
      let copy: &'intern T = self.arena.alloc_copy(data);
      let (idx, _) = self.strintern.insert_full(copy);
      Entry(idx)
    }
  }

  pub fn get(&self, entry: Entry) -> Option<&'intern T> {
    self.strintern.get_index(entry.0).map(|v| &**v)
  }
}

#[cfg(test)]
mod tests {
  use sable_arena::TypedArena;

  use crate::interner::Interner;

  #[test]
  fn test_interner() {
    let arena: TypedArena<str> = TypedArena::new();
    let mut interner = Interner::new(&arena);

    let idx = {
      let str1 = "hello".to_string();
      interner.intern(str1.as_str())
    };

    println!("Interned string: {}", interner.get(idx).unwrap());
  }
}

use std::hash::Hash;

use indexmap::IndexMap;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Interner<K> {
  map: IndexMap<K, usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Interned(pub usize);

impl<K> Interner<K>
where
  K: Eq + Hash,
{
  pub fn new() -> Self {
    Self {
      map: IndexMap::new(),
    }
  }

  pub fn intern(&mut self, key: K) -> Interned {
    if let Some(index) = self.map.get_index_of(&key) {
      Interned(index)
    } else {
      let index = self.map.len();
      self.map.insert(key, index);
      Interned(index)
    }
  }

  pub fn get(&self, interned: &Interned) -> Option<&K> {
    self.map.get_index(interned.0).map(|(key, _)| key)
  }
}

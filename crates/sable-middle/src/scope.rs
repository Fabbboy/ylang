use getset::Getters;
use indexmap::IndexSet;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Symbol {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

#[derive(Debug, Getters)]
pub struct Scope<'scope> {
  #[getset(get = "pub")]
  id: ScopeId,
  #[getset(get = "pub")]
  parent: Option<&'scope Scope<'scope>>,
  #[getset(get = "pub")]
  symbols: IndexSet<&'scope Symbol>,
}

impl<'scope> Scope<'scope> {
  pub fn new(id: ScopeId, parent: Option<&'scope Scope<'scope>>) -> Self {
    Scope {
      id,
      parent,
      symbols: IndexSet::new(),
    }
  }

  pub fn add(&mut self, symbol: &'scope Symbol) {
    self.symbols.insert(symbol);
  }

  pub fn symbol(&self, id: SymbolId) -> Option<&'scope Symbol> {
    if let Some(symbol) = self.symbols.get_index(id.0) {
      Some(symbol)
    } else {
      self.parent.and_then(|parent| parent.symbol(id))
    }
  }
}

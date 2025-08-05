use std::collections::HashMap;

use getset::Getters;
use sable_ast::NodeId;

#[derive(Debug)]
pub enum Symbol {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);

#[derive(Debug, Getters)]
pub struct Scope<'scope> {
  #[getset(get = "pub")]
  id: ScopeId,
  #[getset(get = "pub")]
  parent: Option<&'scope Scope<'scope>>,
  #[getset(get = "pub")]
  symbols: HashMap<NodeId, &'scope Symbol>,
}

impl<'scope> Scope<'scope> {
  pub fn new(id: ScopeId, parent: Option<&'scope Scope<'scope>>) -> Self {
    Scope {
      id,
      parent,
      symbols: HashMap::new(),
    }
  }

  pub fn add(&mut self, id: NodeId, symbol: &'scope Symbol) {
    self.symbols.insert(id, symbol);
  }

  pub fn symbol(&self, id: NodeId) -> Option<&'scope Symbol> {
    self
      .symbols
      .get(&id)
      .cloned()
      .or_else(|| self.parent.and_then(|parent| parent.symbol(id)))
  }
}

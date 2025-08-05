use getset::Getters;
use indexmap::IndexMap;
use sable_arena::TypedArena;
use sable_common::interner::StrInterner;

use crate::scope::{
  Scope,
  ScopeId,
  Symbol,
};

#[derive(Debug, Getters)]
pub struct Context<'ast, 'src> {
  #[getset(get = "pub")]
  intern: &'ast StrInterner<'src>,
  #[getset(get = "pub")]
  scopes: IndexMap<ScopeId, &'ast mut Scope<'ast>>,
  scope_arena: &'ast TypedArena<Scope<'ast>>,
  symbol_arena: &'ast TypedArena<Symbol>,
}

impl<'ast, 'src> Context<'ast, 'src> {
  pub fn new(
    intern: &'ast StrInterner<'src>,
    scope_arena: &'ast TypedArena<Scope<'ast>>,
    symbol_arena: &'ast TypedArena<Symbol>,
  ) -> Self {
    let mut scopes = IndexMap::new();
    let global = scope_arena.alloc(Scope::new(ScopeId(scopes.len()), None));
    scopes.insert(global.id().clone(), global);

    Context {
      intern,
      scopes,
      scope_arena,
      symbol_arena,
    }
  }
}

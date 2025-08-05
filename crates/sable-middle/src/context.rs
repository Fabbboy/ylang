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
pub struct Context<'ctx, 'src> {
  #[getset(get = "pub")]
  intern: &'ctx StrInterner<'src>,
  #[getset(get = "pub")]
  scopes: IndexMap<ScopeId, &'ctx mut Scope<'ctx>>,
  scope_arena: &'ctx TypedArena<Scope<'ctx>>,
  symbol_arena: &'ctx TypedArena<Symbol>,
}

impl<'ctx, 'src> Context<'ctx, 'src> {
  pub fn new(
    intern: &'ctx StrInterner<'src>,
    scope_arena: &'ctx TypedArena<Scope<'ctx>>,
    symbol_arena: &'ctx TypedArena<Symbol>,
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

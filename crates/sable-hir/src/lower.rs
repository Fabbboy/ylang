use std::{
  collections::HashMap,
  hash::{
    DefaultHasher,
    Hash,
    Hasher,
  },
};

use sable_ast::types::Type as AstType;

use crate::{
  context::Context,
  module::{
    DefId,
    ItemId,
    OwnerId,
  },
  ty::{
    TypeId,
    TypeKind,
  },
};

pub struct AstLowerer<'lower, 'hir> {
  context: &'lower mut Context<'hir>,
  ast_type_map: HashMap<AstType<'hir>, TypeId<'hir>>,
  hasher: DefaultHasher,
  current_mod: OwnerId,
}

impl<'lower, 'hir> AstLowerer<'lower, 'hir> {
  pub fn new(context: &'lower mut Context<'hir>) -> Self {
    Self {
      context,
      ast_type_map: HashMap::new(),
      hasher: DefaultHasher::new(),
      current_mod: OwnerId(0),
    }
  }

  fn lower_type(&mut self, ast_type: AstType<'hir>) -> TypeId<'hir> {
    if let Some(type_id) = self.ast_type_map.get(&ast_type) {
      return type_id;
    }

    let type_kind = TypeKind::from_ast(&ast_type, &mut self.context);
    type_kind.hash(&mut self.hasher);
    let type_hashed = self.hasher.finish() as usize;

    let def_id = ItemId(type_hashed);

    todo!()
  }
}

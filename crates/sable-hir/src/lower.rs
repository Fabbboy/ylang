use std::collections::HashMap;

use sable_ast::types::Type as AstType;

use crate::{context::Context, ty::TypeId};

pub struct AstLowerer<'lower, 'hir> {
  context: &'lower Context<'hir>,
  ast_type_map: HashMap<AstType<'hir>, TypeId<'hir>>,
}

impl<'lower, 'hir> AstLowerer<'lower, 'hir> {
  pub fn new(context: &'lower Context<'hir>) -> Self {
    Self {
      context,
      ast_type_map: HashMap::new(),
    }
  }
}

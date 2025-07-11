use crate::context::Context;

pub struct AstLowerer<'lower, 'hir> {
  context: &'lower mut Context<'hir>,
}

impl<'lower, 'hir> AstLowerer<'lower, 'hir> {
  pub fn new(context: &'lower mut Context<'hir>) -> Self {
    Self { context }
  }
}

use sable_ast::ast::Ast;
use sable_hir::package::Package;

pub struct AstLowering<'ast, 'lower, 'hir> {
  asts: &'lower [Ast<'ast>],
  package: &'lower Package<'hir>,
}

impl<'ast, 'lower, 'hir> AstLowering<'ast, 'lower, 'hir> {
  pub fn new(asts: &'lower [Ast<'ast>], package: &'lower Package<'hir>) -> Self {
    Self { asts, package }
  }

  pub fn lower(&self) -> Result<(), ()> {
    Ok(())
  }
}

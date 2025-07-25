use sable_ast::ast::Ast;
use sable_common::writer::Sink;
use sable_hir::package::Package;

enum ResolverStatus {
  Success,
  OhNo,
}

pub struct Resolver<'ast, 'lower, 'hir, D>
where
  D: Sink<'ast>,
{
  ast: &'lower [Ast<'ast>],
  package: &'lower Package<'hir>,
  reporter: &'lower D,
}

impl<'ast, 'lower, 'hir, D> Resolver<'ast, 'lower, 'hir, D>
where
  D: Sink<'ast>,
{
  pub fn new(
    ast: &'lower [Ast<'ast>],
    package: &'lower Package<'hir>,
    reporter: &'lower D,
  ) -> Self {
    Self {
      ast,
      package,
      reporter,
    }
  }

  pub fn resolve(&mut self) -> Result<(), ()> {
    let status = ResolverStatus::Success;

    match status {
      ResolverStatus::Success => Ok(()),
      ResolverStatus::OhNo => Err(()),
    }
  }
}

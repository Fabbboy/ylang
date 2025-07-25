use std::marker::PhantomData;

use sable_ast::ast::Ast;
use sable_common::writer::Sink;
use sable_hir::package::Package;

enum ResolverStatus {
  Success,
  OhNo,
}

pub struct Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  ast: &'lower [Ast<'ast>],
  package: &'lower Package<'hir>,
  reporter: &'lower D,
  _marker: PhantomData<(&'src ())>,
}

impl<'src, 'hir, 'ast, 'lower, D> Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
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
      _marker: PhantomData,
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

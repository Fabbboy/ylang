use std::marker::PhantomData;

use sable_ast::{
  ast::Ast,
  located::Located,
  objects::function::Function,
};
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
  asts: &'lower mut [Ast<'ast>],
  package: &'lower mut Package<'hir>,
  reporter: &'lower mut D,
  _marker: PhantomData<&'src ()>,
}

impl<'src, 'hir, 'ast, 'lower, D> Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  pub fn new(
    asts: &'lower mut [Ast<'ast>], // the slice it self is not mutated only the items. Need mutable to do string interning 
    package: &'lower mut Package<'hir>, // only needed mutable for interning 
    reporter: &'lower mut D,
  ) -> Self {
    Self {
      asts,
      package,
      reporter,
      _marker: PhantomData,
    }
  }

  fn visit_func(&mut self, func: &mut Function<'ast>) -> Result<(), ()> {
    let status = ResolverStatus::Success;

    for param in *func.params_mut() {
      let lexeme = self.package.intern(param.name().value());
      param.set_name(param.name().replace(lexeme));
    }

    match status {
      ResolverStatus::Success => Ok(()),
      ResolverStatus::OhNo => Err(()),
    }
  }

  fn visit_ast(&mut self, ast: &mut Ast<'ast>) -> Result<(), ()> {
    for func in ast.funcs_mut() {
      self.visit_func(func)?;
    }
    Ok(())
  }

  pub fn resolve(&mut self) -> Result<(), ()> {
    let mut status = ResolverStatus::Success;

    for ast in self.asts.iter_mut() {
      if let Err(_) = self.visit_ast(ast) {
        status = ResolverStatus::OhNo;
      }
    }

    match status {
      ResolverStatus::Success => Ok(()),
      ResolverStatus::OhNo => Err(()),
    }
  }
}

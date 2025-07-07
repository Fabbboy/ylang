use std::mem::MaybeUninit;

use sable_ast::{
  ast::Ast,
  expression::Expression,
  objects::function::Function,
  statement::{
    VariableStatement,
    VisitStatement,
  },
};
use sable_common::context::Context;

use crate::{
  module::HirModule,
  objects::function::HirFunction,
};

pub struct AstLowering<'hir, 'ctx> {
  ast: &'hir Ast<'ctx>,
  hir: &'hir mut HirModule<'ctx>,
}

impl<'hir, 'ctx> AstLowering<'hir, 'ctx> {
  pub fn new(ast: &'hir Ast<'ctx>, hir: &'hir mut HirModule<'ctx>) -> Self {
    Self { ast, hir }
  }

  pub fn lower(&mut self) {
    for func in self.ast.funcs().iter() {
      let hir_func = self.lower_func(func);
      self.hir.funcs_mut().push(hir_func);
    }
  }

  fn lower_func(&mut self, func: &Function<'ctx>) -> HirFunction<'ctx> {
    todo!()
  }
}

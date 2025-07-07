use std::{
  mem::MaybeUninit,
  slice,
};

use sable_ast::{
  ast::Ast,
  objects::function::Function,
};
use sable_common::context::Context;

use crate::{
  module::HirModule,
  objects::function::HirFunction,
};

pub struct AstLowering<'lower, 'hir> {
  ast: &'lower Ast<'hir>,
  hir: &'lower mut HirModule<'hir>,
  ctx: &'lower Context,
}

impl<'lower, 'hir> AstLowering<'lower, 'hir> {
  pub fn new(ast: &'lower Ast<'hir>, hir: &'lower mut HirModule<'hir>, ctx: &'hir Context) -> Self {
    Self { ast, hir, ctx }
  }

  pub fn lower(&mut self) {
    let funcs_uninit = self
      .ctx
      .hir_bump()
      .alloc_slice_fill_with::<MaybeUninit<&'hir HirFunction<'hir>>, _>(
        self.ast.funcs().len(),
        |_| MaybeUninit::uninit(),
      );

    for (idx, func) in self.ast.funcs().iter().enumerate() {
      let hir_func = self.lower_func(func);
      if let Some(func_slot) = funcs_uninit.get_mut(idx) {
        *func_slot = MaybeUninit::new(hir_func);
      }
    }

    let funcs_slice = unsafe {
      slice::from_raw_parts(
        funcs_uninit.as_ptr() as *const &'hir HirFunction<'hir>,
        funcs_uninit.len(),
      )
    };

    self.hir.set_funcs(funcs_slice);
  }

  fn lower_func(&mut self, func: &Function<'hir>) -> &'hir HirFunction<'hir> {
    todo!()
  }
}

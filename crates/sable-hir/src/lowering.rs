use std::{
  mem::MaybeUninit,
  slice,
};

use sable_ast::{
  ast::Ast,
  objects::function::{
    Function,
    FunctionParam,
  },
};

use crate::{
  module::HirModule,
  objects::function::{
    HirFunction,
    HirParam,
  },
};

pub struct AstLowering<'lower, 'hir> {
  ast: &'lower Ast<'hir>,
}

impl<'lower, 'hir> AstLowering<'lower, 'hir> {
  pub fn new(ast: &'lower Ast<'hir>) -> Self {
    Self { ast }
  }

  pub fn lower(&mut self) -> HirModule<'hir> {
    let mut hir = HirModule::new();

    let funcs_uninit = hir
      .hir_bump()
      .alloc_slice_fill_with::<MaybeUninit<&'hir HirFunction<'hir>>, _>(
        self.ast.funcs().len(),
        |_| MaybeUninit::uninit(),
      );

    self.lower_funcs(self.ast.funcs().as_slice(), funcs_uninit, &mut hir);

    let func_slice = unsafe {
      slice::from_raw_parts(
        funcs_uninit.as_ptr() as *const &'hir HirFunction<'hir>,
        funcs_uninit.len(),
      )
    };

    hir.set_funcs(func_slice);
    hir
  }

  fn lower_funcs(
    &mut self,
    ast_funcs: &[Function<'hir>],
    funcs: &mut [MaybeUninit<&HirFunction<'hir>>],
    hir: &mut HirModule<'hir>,
  ) {
    for (idx, func) in ast_funcs.iter().enumerate() {
      let hir_func = self.lower_func(func, hir);
      if let Some(func_slot) = funcs.get_mut(idx) {
        *func_slot = MaybeUninit::new(hir_func);
      }
    }
  }

  fn lower_param(
    &mut self,
    param: &FunctionParam<'hir>,
    hir: &HirModule<'hir>,
  ) -> &'hir HirParam<'hir> {
    let name_sym = hir.symbols().intern(param.name());

    todo!()
  }

  fn lower_func(
    &mut self,
    func: &Function<'hir>,
    hir: &mut HirModule<'hir>,
  ) -> &'hir HirFunction<'hir> {
    let param_slice = hir
      .hir_bump()
      .alloc_slice_fill_with::<MaybeUninit<&'hir HirParam<'hir>>, _>(func.params().len(), |_| {
        MaybeUninit::uninit()
      });

    for (idx, param) in func.params().iter().enumerate() {
      let hir_param = self.lower_param(param, hir);
      if let Some(param_slot) = param_slice.get_mut(idx) {
        *param_slot = MaybeUninit::new(hir_param);
      }
    }

    todo!()
  }
}

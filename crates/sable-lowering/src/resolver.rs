use std::cell::Cell;

use sable_ast::{
  ast::Ast,
  objects::function::Function,
};

enum Status {
  Ok,
  Error,
}

pub struct Resolver<'ast, 'resolve> {
  asts: &'resolve mut [&'ast mut Ast<'ast>],
  id: Cell<usize>,
}

impl<'ast, 'resolve> Resolver<'ast, 'resolve> {
  pub fn new(asts: &'resolve mut [&'ast mut Ast<'ast>]) -> Self {
    Resolver {
      asts,
      id: Cell::new(0),
    }
  }

  fn next_id(&self) -> usize {
    let id = self.id.get();
    self.id.set(id + 1);
    id
  }

  fn resolve_func(&mut self, func: &mut Function<'ast>) -> Result<(), ()> {
    Ok(())
  }

  fn resolve_ast(&mut self, ast: &mut Ast<'ast>) -> Result<(), ()> {
    let mut status = Status::Ok;
    for funcs in ast.funcs_mut() {
      if let Err(_) = self.resolve_func(funcs) {
        status = Status::Error;
      }
    }

    match status {
      Status::Ok => Ok(()),
      Status::Error => Err(()),
    }
  }

  pub fn resolve(&mut self) -> Result<(), ()> {
    let mut status = Status::Ok;

    let asts = std::mem::take(&mut self.asts);

    for ast in asts.iter_mut() {
      match self.resolve_ast(ast) {
        Ok(_) => {}
        Err(_) => {
          status = Status::Error;
        }
      }
    }

    self.asts = asts;

    match status {
      Status::Ok => Ok(()),
      Status::Error => Err(()),
    }
  }
}

use std::cell::Cell;

use sable_ast::{
  ast::Ast,
  expression::{
    AssignExpression,
    BinaryExpression,
    BlockExpression,
    Expression,
    ExpressionVisitorMut,
    IdentifierExpression,
    LiteralExpression,
  },
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
    if let Some(block) = func.block_mut() {}

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

impl<'ast, 'resolve> ExpressionVisitorMut<'ast> for Resolver<'ast, 'resolve> {
  type VisitReturn = Result<(), ()>;

  fn visit_block_mut(
    &mut self,
    block: &mut BlockExpression<'ast>,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    todo!()
  }

  fn visit_literal_mut(
    &mut self,
    literal: &mut LiteralExpression,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    todo!()
  }

  fn visit_assign_mut(
    &mut self,
    assign: &mut AssignExpression<'ast>,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    todo!()
  }

  fn visit_binary_mut(
    &mut self,
    binary: &mut BinaryExpression<'ast>,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    todo!()
  }

  fn visit_identifier_mut(
    &mut self,
    identifier: &mut IdentifierExpression,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    todo!()
  }
}

use std::cell::Cell;

use sable_ast::{
  NodeId,
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
  statement::{
    Statement,
    StatementVisitorMut,
    VariableStatement,
  },
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

  // Used for blocks that are not part of an expression and do not carry an ID nor a Expression object.
  fn visit_block(&mut self, block: &mut BlockExpression<'ast>) -> Result<(), ()> {
    for stmt in block.body_mut() {
      self.visit_stmt_mut(stmt)?;
    }
    Ok(())
  }

  fn resolve_func(&mut self, func: &mut Function<'ast>) -> Result<(), ()> {
    if let Some(block) = func.block_mut() {
      self.visit_block(block)?;
    }

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
    let id = self.next_id();
    _ = expr.id_mut().init(NodeId(id));
    for stmt in block.body_mut() {
      self.visit_stmt_mut(stmt)?;
    }
    Ok(())
  }

  fn visit_literal_mut(
    &mut self,
    literal: &mut LiteralExpression,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = expr.id_mut().init(NodeId(id));
    Ok(())
  }

  fn visit_assign_mut(
    &mut self,
    assign: &mut AssignExpression<'ast>,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = expr.id_mut().init(NodeId(id));
    self.visit_expr_mut(assign.value_mut())?;
    Ok(())
  }

  fn visit_binary_mut(
    &mut self,
    binary: &mut BinaryExpression<'ast>,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = expr.id_mut().init(NodeId(id));
    self.visit_expr_mut(binary.lhs_mut())?;
    self.visit_expr_mut(binary.rhs_mut())?;

    Ok(())
  }

  fn visit_identifier_mut(
    &mut self,
    identifier: &mut IdentifierExpression,
    expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = expr.id_mut().init(NodeId(id));
    Ok(())
  }
}

impl<'ast, 'resolve> StatementVisitorMut<'ast> for Resolver<'ast, 'resolve> {
  type VisitReturn = Result<(), ()>;

  fn visit_expression_mut(
    &mut self,
    expr: &mut Expression<'ast>,
    statement: &mut Statement<'ast>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = statement.id_mut().init(NodeId(id));
    self.visit_expr_mut(expr)?;
    Ok(())
  }

  fn visit_variable_mut(
    &mut self,
    variable: &mut VariableStatement<'ast>,
    statement: &mut Statement<'ast>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = statement.id_mut().init(NodeId(id));
    self.visit_expr_mut(variable.initializer_mut())?;
    Ok(())
  }
}

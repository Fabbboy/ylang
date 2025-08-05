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
use sable_middle::context::Context;

enum Status {
  Ok,
  Error,
}

pub struct Resolver<'ast, 'src, 'resolve> {
  asts: &'resolve mut [&'ast mut Ast<'ast, 'src>],
  id: Cell<usize>,
  context: &'resolve mut Context<'resolve, 'src>,
}

impl<'ast, 'src, 'resolve> Resolver<'ast, 'src, 'resolve> {
  pub fn new(
    asts: &'resolve mut [&'ast mut Ast<'ast, 'src>],
    context: &'resolve mut Context<'resolve, 'src>,
  ) -> Self {
    Resolver {
      asts,
      id: Cell::new(0),
      context,
    }
  }

  fn next_id(&self) -> usize {
    let id = self.id.get();
    self.id.set(id + 1);
    id
  }

  // Used for blocks that are not part of an expression and do not carry an ID nor a Expression object.
  fn visit_block(&mut self, block: &mut BlockExpression<'ast, 'src>) -> Result<(), ()> {
    for stmt in block.body_mut() {
      <Self as StatementVisitorMut>::visit_stmt_mut(self, stmt)?;
    }
    Ok(())
  }

  fn resolve_func(&mut self, func: &mut Function<'ast, 'src>) -> Result<(), ()> {
    if let Some(block) = func.block_mut() {
      self.visit_block(block)?;
    }

    Ok(())
  }

  fn resolve_ast(&mut self, ast: &mut Ast<'ast, 'src>) -> Result<(), ()> {
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

impl<'ast, 'src, 'resolve> ExpressionVisitorMut<'ast, 'src> for Resolver<'ast, 'src, 'resolve> {
  type VisitReturn = Result<(), ()>;

  fn visit_block_mut(
    &mut self,
    block: &mut BlockExpression<'ast, 'src>,
    expr: &mut Expression<'ast, 'src>,
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
    expr: &mut Expression<'ast, 'src>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = expr.id_mut().init(NodeId(id));
    Ok(())
  }

  fn visit_assign_mut(
    &mut self,
    assign: &mut AssignExpression<'ast, 'src>,
    expr: &mut Expression<'ast, 'src>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = expr.id_mut().init(NodeId(id));
    self.visit_expr_mut(assign.value_mut())?;
    Ok(())
  }

  fn visit_binary_mut(
    &mut self,
    binary: &mut BinaryExpression<'ast, 'src>,
    expr: &mut Expression<'ast, 'src>,
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
    expr: &mut Expression<'ast, 'src>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = expr.id_mut().init(NodeId(id));
    Ok(())
  }
}

impl<'ast, 'src, 'resolve> StatementVisitorMut<'ast, 'src> for Resolver<'ast, 'src, 'resolve> {
  type VisitReturn = Result<(), ()>;

  fn visit_expression_mut(
    &mut self,
    expr: &mut Expression<'ast, 'src>,
    statement: &mut Statement<'ast, 'src>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = statement.id_mut().init(NodeId(id));
    self.visit_expr_mut(expr)?;
    Ok(())
  }

  fn visit_variable_mut(
    &mut self,
    variable: &mut VariableStatement<'ast, 'src>,
    statement: &mut Statement<'ast, 'src>,
  ) -> Self::VisitReturn {
    let id = self.next_id();
    _ = statement.id_mut().init(NodeId(id));
    self.visit_expr_mut(variable.initializer_mut())?;
    Ok(())
  }
}

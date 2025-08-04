#![allow(clippy::result_unit_err)]

use std::{
  marker::PhantomData,
  mem,
};

use sable_ast::{
  NodeId,
  ast::Ast,
  expression::{
    AssignExpression,
    BinaryExpression,
    BlockExpression,
    IdentifierExpression,
    LiteralExpression,
    VisitExpressionMut,
  },
  objects::function::Function,
  statement::{
    VariableStatement,
    VisitStatementMut,
  },
};
use sable_common::{
  location::Location,
  once::Once,
  writer::Sink,
};
use sable_hir::package::Package;

enum ResolverStatus {
  Success,
  OhNo,
}

pub struct Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  asts: &'lower mut [&'ast mut Ast<'ast>],
  id: usize,
  _package: &'lower mut Package<'hir>,
  _reporter: &'lower mut D,
  _marker: PhantomData<&'src ()>,
}

impl<'src, 'hir, 'ast, 'lower, D> Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  pub fn new(
    asts: &'lower mut [&'ast mut Ast<'ast>],
    package: &'lower mut Package<'hir>,
    reporter: &'lower mut D,
  ) -> Self {
    Self {
      asts,
      id: 0,
      _package: package,
      _reporter: reporter,
      _marker: PhantomData,
    }
  }

  fn get_inc(&mut self) -> usize {
    let id = self.id;
    self.id += 1;
    return id;
  }

  fn visit_func(&mut self, func: &mut Function<'ast>) -> Result<(), ()> {
    let status = ResolverStatus::Success;

    if let Some(body) = func.block_mut() {
      let mut dummy = Once::<NodeId>::Uninit;
      let dummy_loc = Location::new(0..0, "dummy");
      self.visit_block(&mut dummy, body, &dummy_loc);
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
    let asts = mem::replace(&mut self.asts, &mut []);

    for ast in asts.iter_mut() {
      if self.visit_ast(ast).is_err() {
        status = ResolverStatus::OhNo;
      }
    }

    self.asts = asts;

    match status {
      ResolverStatus::Success => Ok(()),
      ResolverStatus::OhNo => Err(()),
    }
  }
}

impl<'src, 'hir, 'ast, 'lower, D> VisitStatementMut<'ast> for Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  fn visit_variable(
    &mut self,
    id: &mut Once<NodeId>,
    variable_statement: &mut VariableStatement<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    _ = id.init(NodeId(self.get_inc()));
    self.visit_expression(variable_statement.initializer_mut());
  }
}

impl<'src, 'hir, 'ast, 'lower, D> VisitExpressionMut<'ast> for Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  type Ret = ();

  fn visit_block(
    &mut self,
    id: &mut Once<NodeId>,
    block: &mut BlockExpression<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    _ = id.init(NodeId(self.get_inc()));
    for stmt in block.body_mut().iter_mut() {
      self.visit_statement(stmt);
    }
  }

  fn visit_literal(
    &mut self,
    id: &mut Once<NodeId>,
    _literal: &mut LiteralExpression,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    _ = id.init(NodeId(self.get_inc()));
  }

  fn visit_assign(
    &mut self,
    id: &mut Once<NodeId>,
    assign: &mut AssignExpression<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    _ = id.init(NodeId(self.get_inc()));
    self.visit_expression(assign.value_mut());
  }

  fn visit_binary(
    &mut self,
    id: &mut Once<NodeId>,
    binary: &mut BinaryExpression<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    _ = id.init(NodeId(self.get_inc()));
    match binary {
      BinaryExpression::Add(inner) => {
        self.visit_expression(inner.left_mut());
        self.visit_expression(inner.right_mut());
      }
      BinaryExpression::Subtract(inner) => {
        self.visit_expression(inner.left_mut());
        self.visit_expression(inner.right_mut());
      }
      BinaryExpression::Multiply(inner) => {
        self.visit_expression(inner.left_mut());
        self.visit_expression(inner.right_mut());
      }
      BinaryExpression::Divide(inner) => {
        self.visit_expression(inner.left_mut());
        self.visit_expression(inner.right_mut());
      }
    }
  }

  fn visit_identifier(
    &mut self,
    id: &mut Once<NodeId>,
    _identifier: &mut IdentifierExpression,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    _ = id.init(NodeId(self.get_inc()));
  }
}

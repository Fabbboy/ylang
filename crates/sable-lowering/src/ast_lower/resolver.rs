#![allow(clippy::result_unit_err)]

use std::marker::PhantomData;

use sable_ast::{
  NodeId,
  ast::Ast,
  expression::{
    AssignExpression,
    BinaryExpression,
    BlockExpression,
    Expression,
    IdentifierExpression,
    LiteralExpression,
    VisitExpression,
  },
  objects::function::Function,
  statement::{
    VariableStatement,
    VisitStatement,
  },
};
use sable_common::{
  location::Location,
  once::Once,
  writer::Sink,
};
use sable_hir::package::Package;

use crate::ast_lower_error::AstLoweringError;

enum ResolverStatus {
  Success,
  OhNo,
}

pub struct Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  asts: &'lower [&'lower mut Ast<'ast>],
  _package: &'lower mut Package<'hir>,
  _reporter: &'lower mut D,
  _marker: PhantomData<&'src ()>,
}

impl<'src, 'hir, 'ast, 'lower, D> Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  pub fn new(
    asts: &'lower [&'lower mut Ast<'ast>],
    package: &'lower mut Package<'hir>,
    reporter: &'lower mut D,
  ) -> Self {
    Self {
      asts,
      _package: package,
      _reporter: reporter,
      _marker: PhantomData,
    }
  }

  fn visit_func(&mut self, _func: &Function<'ast>) -> Result<(), ()> {
    let status = ResolverStatus::Success;

    match status {
      ResolverStatus::Success => Ok(()),
      ResolverStatus::OhNo => Err(()),
    }
  }

  fn visit_ast(&mut self, ast: &Ast<'ast>) -> Result<(), ()> {
    for func in ast.funcs() {
      self.visit_func(func)?;
    }
    Ok(())
  }

  pub fn resolve(&mut self) -> Result<(), ()> {
    let mut status = ResolverStatus::Success;

    for ast in self.asts.iter() {
      if self.visit_ast(ast).is_err() {
        status = ResolverStatus::OhNo;
      }
    }

    match status {
      ResolverStatus::Success => Ok(()),
      ResolverStatus::OhNo => Err(()),
    }
  }
}

impl<'src, 'hir, 'ast, 'lower, D> VisitExpression<'ast> for Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  type Ret = Result<(), AstLoweringError>;

  fn visit_block(
    &mut self,
    _id: &Once<NodeId>,
    _block: &BlockExpression<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_literal(
    &mut self,
    _id: &Once<NodeId>,
    _literal: &LiteralExpression,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_assign(
    &mut self,
    _id: &Once<NodeId>,
    _assign: &AssignExpression<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_binary(
    &mut self,
    _id: &Once<NodeId>,
    _binary: &BinaryExpression<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_identifier(
    &mut self,
    _id: &Once<NodeId>,
    _identifier: &IdentifierExpression,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    todo!()
  }
}

impl<'src, 'hir, 'ast, 'lower, D> VisitStatement<'ast> for Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  type Ret = Result<(), AstLoweringError>;

  fn visit_expression(
    &mut self,
    _id: &Once<NodeId>,
    _expression: &Expression<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_variable(
    &mut self,
    _id: &Once<NodeId>,
    _variable_statement: &VariableStatement<'ast>,
    _location: &Location<'ast>,
  ) -> Self::Ret {
    todo!()
  } // modifys ast and package does not have to return
}

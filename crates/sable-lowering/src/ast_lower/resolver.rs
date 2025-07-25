use std::marker::PhantomData;

use sable_ast::{
  ast::Ast,
  expression::VisitExpression,
  located::Located,
  objects::function::Function,
  statement::VisitStatement,
};
use sable_common::writer::Sink;
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

impl<'src, 'hir, 'ast, 'lower, D> VisitExpression<'ast> for Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  type Ret = Result<(), AstLoweringError>; // modifys ast and package does not have to return 

  fn visit_block(
    &mut self,
    block: &Located<'ast, sable_ast::expression::BlockExpression<'ast>>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_literal(
    &mut self,
    literal: &Located<'ast, sable_ast::expression::LiteralExpression<'ast>>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_assign(
    &mut self,
    assign: &Located<'ast, sable_ast::expression::AssignExpression<'ast>>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_binary(
    &mut self,
    binary: &Located<'ast, sable_ast::expression::BinaryExpression<'ast>>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_identifier(
    &mut self,
    identifier: &Located<'ast, sable_ast::expression::IdentifierExpression<'ast>>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_expression(
    &mut self,
    expression: &sable_ast::expression::Expression<'ast>,
  ) -> Self::Ret {
    match expression {
      sable_ast::expression::Expression::Block(block) => self.visit_block(block),
      sable_ast::expression::Expression::Literal(literal) => self.visit_literal(literal),
      sable_ast::expression::Expression::Assign(assign) => self.visit_assign(assign),
      sable_ast::expression::Expression::Binary(binary) => self.visit_binary(binary),
      sable_ast::expression::Expression::Identifier(identifier) => {
        self.visit_identifier(identifier)
      }
    }
  }
}

impl<'src, 'hir, 'ast, 'lower, D> VisitStatement<'ast> for Resolver<'src, 'hir, 'ast, 'lower, D>
where
  D: Sink<'src>,
{
  type Ret = Result<(), AstLoweringError>;

  fn visit_expression(
    &mut self,
    expression: &sable_ast::expression::Expression<'ast>,
  ) -> Self::Ret {
    todo!()
  }

  fn visit_variable_statement(
    &mut self,
    variable_statement: &Located<'ast, sable_ast::statement::VariableStatement<'ast>>,
  ) -> Self::Ret {
    todo!()
  } // modifys ast and package does not have to return
}

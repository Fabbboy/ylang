use crate::{
  expression::Expression,
  located::Located,
  statement::{
    Statement, StatementVisitor, StatementVisitorMut, VisitableStmt, VisitableStmtMut
  },
  types::Type,
};
use getset::{
  Getters,
  MutGetters,
};
use sable_common::interner::Entry;
use typed_builder::TypedBuilder;

#[derive(Debug, Getters, MutGetters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VariableStatement<'ctx> {
  #[getset(get = "pub")]
  name: Located<'ctx, Entry>,
  #[getset(get = "pub", get_mut = "pub")]
  initializer: Expression<'ctx>,
  #[getset(get = "pub")]
  type_: Located<'ctx, Type<'ctx>>,
}

impl<'ast> VisitableStmt<'ast> for VariableStatement<'ast> {
  fn accept<V>(&self, statement: &Statement<'ast>, visitor: &mut V) -> V::VisitReturn
  where
    V: StatementVisitor<'ast>,
  {
    visitor.visit_variable(self, statement)
  }
}

impl<'ast> VisitableStmtMut<'ast> for VariableStatement<'ast> {
  fn accept_mut<V>(&mut self, statement: &mut Statement<'ast>, visitor: &mut V) -> V::VisitReturn
  where
    V: StatementVisitorMut<'ast>,
  {
    visitor.visit_variable_mut(self, statement)
  }
}
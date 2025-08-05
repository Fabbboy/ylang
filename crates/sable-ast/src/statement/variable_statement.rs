use crate::{
  expression::Expression,
  located::Located,
  statement::{
    Statement, StatementVisitor, StatementVisitorMut, VisitableStmt, VisitableStmtMut,
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
pub struct VariableStatement<'ast, 'src> {
  #[getset(get = "pub")]
  name: Located<'src, Entry>,
  #[getset(get = "pub", get_mut = "pub")]
  initializer: Expression<'ast, 'src>,
  #[getset(get = "pub")]
  type_: Located<'src, Type<'src>>,
}

impl<'ast, 'src> VisitableStmt<'ast, 'src> for VariableStatement<'ast, 'src> {
  fn accept<V>(&self, statement: &Statement<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: StatementVisitor<'ast, 'src>,
  {
    visitor.visit_variable(self, statement)
  }
}

impl<'ast, 'src> VisitableStmtMut<'ast, 'src> for VariableStatement<'ast, 'src> {
  fn accept_mut<V>(&mut self, statement: &mut Statement<'ast, 'src>, visitor: &mut V) -> V::VisitReturn
  where
    V: StatementVisitorMut<'ast, 'src>,
  {
    visitor.visit_variable_mut(self, statement)
  }
}
use crate::{
  NodeId,
  expression::Expression,
};

pub mod variable_statement;

use getset::{
  Getters,
  MutGetters,
};
use sable_common::{
  location::Location,
  once::Once,
};
use typed_builder::TypedBuilder;
pub use variable_statement::VariableStatement;

#[derive(Debug, TypedBuilder, Getters, MutGetters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Statement<'ctx> {
  #[getset(get = "pub")]
  location: Location<'ctx>,
  #[getset(get = "pub", get_mut = "pub")]
  kind: StatementKind<'ctx>,
  #[getset(get = "pub", get_mut = "pub")]
  #[builder(default)]
  id: Once<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StatementKind<'ctx> {
  Expression(Expression<'ctx>),
  Variable(VariableStatement<'ctx>),
}

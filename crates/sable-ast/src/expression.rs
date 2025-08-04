pub mod assign_expression;
pub mod binary_expression;
pub mod block_expression;
pub mod identifier_expression;
pub mod literal_expression;

pub use assign_expression::AssignExpression;
pub use binary_expression::BinaryExpression;
pub use block_expression::BlockExpression;
use getset::{
  Getters,
  MutGetters,
};
pub use identifier_expression::IdentifierExpression;
pub use literal_expression::LiteralExpression;
use typed_builder::TypedBuilder;

use crate::NodeId;
use sable_common::{
  location::Location,
  once::Once,
};

#[derive(Debug, Getters, MutGetters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Expression<'ctx> {
  #[getset(get = "pub")]
  location: Location<'ctx>,
  #[getset(get = "pub", get_mut = "pub")]
  kind: ExpressionKind<'ctx>,
  #[getset(get = "pub", get_mut = "pub")]
  #[builder(default)]
  id: Once<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ExpressionKind<'ctx> {
  Block(BlockExpression<'ctx>),
  Literal(LiteralExpression),
  Assign(AssignExpression<'ctx>),
  Binary(BinaryExpression<'ctx>),
  Identifier(IdentifierExpression),
}

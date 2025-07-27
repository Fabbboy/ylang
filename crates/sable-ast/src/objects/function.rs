use getset::{
  Getters,
  Setters,
};
use typed_builder::TypedBuilder;

use crate::{
  expression::block_expression::BlockExpression,
  located::Located,
  types::{
    Type,
    TypeNamePair,
  },
};
use sable_common::interner::Entry;

pub const MAX_INLINE_PARAMS: usize = 6;

#[derive(Getters, Setters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FunctionParam<'ctx> {
  #[getset(get = "pub", set = "pub")]
  name: Located<'ctx, Entry>,
  #[getset(get = "pub")]
  type_: Located<'ctx, Type<'ctx>>,
}

impl<'ctx> From<Located<'ctx, TypeNamePair<'ctx>>> for FunctionParam<'ctx> {
  fn from(pair: Located<'ctx, TypeNamePair<'ctx>>) -> Self {
    Self {
      name: Located::builder()
        .value(pair.value().name().clone())
        .location(pair.location().clone())
        .build(),
      type_: Located::builder()
        .value(pair.value().type_().clone())
        .location(pair.location().clone())
        .build(),
    }
  }
}

#[derive(Getters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Function<'ctx> {
  #[getset(get = "pub")]
  name: Located<'ctx, Entry>,
  #[getset(get = "pub")]
  params: &'ctx [FunctionParam<'ctx>],
  #[getset(get = "pub")]
  return_type: Located<'ctx, Type<'ctx>>,
  #[getset(get = "pub")]
  block: Option<BlockExpression<'ctx>>,
}

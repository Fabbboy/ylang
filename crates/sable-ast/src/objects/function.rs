use getset::{
  Getters,
  MutGetters,
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
pub struct FunctionParam<'src> {
  #[getset(get = "pub", set = "pub")]
  name: Located<'src, Entry>,
  #[getset(get = "pub")]
  type_: Located<'src, Type<'src>>,
}

impl<'src> From<Located<'src, TypeNamePair<'src>>> for FunctionParam<'src> {
  fn from(pair: Located<'src, TypeNamePair<'src>>) -> Self {
    Self {
      name: Located::builder()
        .value(*pair.value().name())
        .location(pair.location().clone())
        .build(),
      type_: Located::builder()
        .value(pair.value().type_().clone())
        .location(pair.location().clone())
        .build(),
    }
  }
}

#[derive(Getters, MutGetters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Function<'ast, 'src> {
  #[getset(get = "pub")]
  name: Located<'src, Entry>,
  #[getset(get = "pub")]
  params: &'ast [FunctionParam<'src>],
  #[getset(get = "pub")]
  return_type: Located<'src, Type<'src>>,
  #[getset(get = "pub", get_mut = "pub")]
  block: Option<BlockExpression<'ast, 'src>>,
}

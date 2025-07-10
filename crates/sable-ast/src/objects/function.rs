use getset::Getters;
use sable_common::location::Location;
use smallvec::SmallVec;
use typed_builder::TypedBuilder;

use crate::{
  expression::block_expression::BlockExpression,
  types::{
    Type,
    TypeNamePair,
  },
};

pub const MAX_INLINE_PARAMS: usize = 6;

#[derive(Getters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FunctionParam<'ctx> {
  #[getset(get = "pub")]
  name: &'ctx str,
  #[getset(get = "pub")]
  type_: Type<'ctx>,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

impl<'ctx> From<TypeNamePair<'ctx>> for FunctionParam<'ctx> {
  fn from(pair: TypeNamePair<'ctx>) -> Self {
    Self {
      name: pair.name(),
      type_: pair.type_().clone(),
      location: pair.location().clone(),
    }
  }
}

#[derive(Getters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Function<'ctx> {
  #[getset(get = "pub")]
  name: &'ctx str,
  #[getset(get = "pub")]
  params: SmallVec<[FunctionParam<'ctx>; MAX_INLINE_PARAMS]>,
  #[getset(get = "pub")]
  return_type: Type<'ctx>,
  #[getset(get = "pub")]
  location: Location<'ctx>,
  #[getset(get = "pub")]
  block: Option<BlockExpression<'ctx>>,
}

use std::rc::Rc;

use getset::Getters;
use smallvec::SmallVec;
use typed_builder::TypedBuilder;

use crate::{
  expression::block_expression::BlockExpression,
  location::Location,
  types::{
    Type,
    TypeNamePair,
  },
};

pub const MAX_INLINE_PARAMS: usize = 6;

#[derive(Getters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FunctionParam {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  type_: Type,
  #[getset(get = "pub")]
  location: Location,
}

impl From<TypeNamePair> for FunctionParam {
  fn from(pair: TypeNamePair) -> Self {
    Self {
      name: pair.name().clone(),
      type_: pair.type_().clone(),
      location: pair.location().clone(),
    }
  }
}

#[derive(Getters, TypedBuilder, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Function<'ctx> {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  params: SmallVec<[FunctionParam; MAX_INLINE_PARAMS]>,
  #[getset(get = "pub")]
  return_type: Type,
  #[getset(get = "pub")]
  location: Location,
  #[getset(get = "pub")]
  block: Option<BlockExpression<'ctx>>,
}

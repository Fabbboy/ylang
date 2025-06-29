use std::rc::Rc;

use getset::Getters;
use smallvec::SmallVec;
use typed_builder::TypedBuilder;

use crate::types::Type;

pub const MAX_INLINE_PARAMS: usize = 4;

#[derive(Getters, TypedBuilder)]
pub struct FunctionParam {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  type_: Type,
}

#[derive(Getters, TypedBuilder)]
pub struct Function {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  params: SmallVec<[FunctionParam; MAX_INLINE_PARAMS]>,
  #[getset(get = "pub")]
  return_type: Type,
}

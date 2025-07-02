use std::rc::Rc;

use getset::Getters;
#[cfg(feature = "serde")]
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::location::Location;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum PrimitiveType {
  I8,
  I16,
  I32,
  F32,
  F64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Type {
  Inference,
  Primitive(PrimitiveType),
  Custom(Rc<str>),
  Pointer(Box<Type>),
}

impl From<PrimitiveType> for Type {
  fn from(primitive_type: PrimitiveType) -> Self {
    Type::Primitive(primitive_type)
  }
}

#[derive(TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TypeNamePair {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  type_: Type,
  #[getset(get = "pub")]
  location: Location,
}

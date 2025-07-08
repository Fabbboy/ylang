use getset::Getters;
use typed_builder::TypedBuilder;

use crate::location::Location;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PrimitiveType {
  I8,
  I16,
  I32,
  F32,
  F64,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Type<'ctx> {
  #[default]
  Inference,
  Primitive(PrimitiveType),
  Custom(&'ctx str),
  Pointer(Box<Type<'ctx>>),
}

impl<'ctx> From<PrimitiveType> for Type<'ctx> {
  fn from(primitive_type: PrimitiveType) -> Self {
    Type::Primitive(primitive_type)
  }
}

#[derive(TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeNamePair<'ctx> {
  #[getset(get = "pub")]
  name: &'ctx str,
  #[getset(get = "pub")]
  type_: Type<'ctx>,
  #[getset(get = "pub")]
  location: Location,
}

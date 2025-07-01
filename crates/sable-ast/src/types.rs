use std::rc::Rc;

use getset::Getters;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::location::Location;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum PrimitiveType {
  I32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Type {
  Invalid,
  Primitive(PrimitiveType),
  Custom(Rc<str>),
}

impl From<PrimitiveType> for Type {
  fn from(primitive_type: PrimitiveType) -> Self {
    Type::Primitive(primitive_type)
  }
}

#[derive(TypedBuilder, Getters)]
pub struct TypeNamePair {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  type_: Type,
  #[getset(get = "pub")]
  location: Location,
}

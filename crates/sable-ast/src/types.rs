use getset::Getters;
use typed_builder::TypedBuilder;

use crate::location::Location;

#[derive(Clone, Debug, PartialEq, Eq, Default, TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path<'ctx> {
  #[getset(get = "pub")]
  segments: Vec<&'ctx str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Type<'ctx> {
  #[default]
  Infer,
  Path(Path<'ctx>),
  Pointer(Box<Type<'ctx>>),
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

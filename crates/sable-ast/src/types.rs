use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

use crate::located::Located;
use sable_common::interner::Entry;

#[derive(Clone, Debug, PartialEq, Eq, Default, TypedBuilder, Getters, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path<'ctx> {
  #[getset(get = "pub")]
  segments: Vec<Located<'ctx, Entry>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
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
  name: Entry,
  #[getset(get = "pub")]
  type_: Type<'ctx>,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

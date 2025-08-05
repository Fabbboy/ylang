use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;

use crate::located::Located;
use sable_common::interner::Entry;

#[derive(Clone, Debug, PartialEq, Eq, Default, TypedBuilder, Getters, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path<'src> {
  #[getset(get = "pub")]
  segments: Vec<Located<'src, Entry>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Type<'src> {
  #[default]
  Infer,
  Path(Path<'src>),
  Pointer(Box<Type<'src>>),
}

#[derive(TypedBuilder, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeNamePair<'src> {
  #[getset(get = "pub")]
  name: Entry,
  #[getset(get = "pub")]
  type_: Type<'src>,
  #[getset(get = "pub")]
  location: Location<'src>,
}

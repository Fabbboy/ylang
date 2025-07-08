use crate::{
  location::Location,
  ty::{
    def::Definition,
    resolution::Resolution,
  },
};

#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]

pub enum TypeKind {
  Resolution(Resolution),
  Pointer(Box<TypeKind>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]

pub struct Type {
  id: Definition,
  kind: TypeKind,
  location: Location,
}

pub type Ty<'hir> = &'hir Type;

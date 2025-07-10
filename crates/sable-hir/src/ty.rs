use getset::Getters;
use typed_builder::TypedBuilder;

use crate::module::DefId;

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TypeKind {
  None,
}

#[derive(Debug, Getters, PartialEq, Hash, Eq, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Type {
  id: DefId,
  kind: TypeKind,
}

pub type TypeId<'hir> = &'hir Type;

use getset::Getters;
use sable_arena::interner::Interner;
use typed_builder::TypedBuilder;

use crate::item::DefId;

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
pub type TypeInterner<'hir> = Interner<'hir, Type>;

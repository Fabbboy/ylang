use getset::Getters;
use sable_common::interner::Interned;
use typed_builder::TypedBuilder;

use crate::module::DefId;

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path<'hir> {
  segments: &'hir [Interned],
}

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TypeKind<'hir> {
  Infer,
  Path(Path<'hir>),
  Pointer(&'hir TypeId<'hir>),
}

#[derive(Debug, Getters, PartialEq, Hash, Eq, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Type<'hir> {
  id: DefId,
  kind: TypeKind<'hir>,
}

pub type TypeId<'hir> = &'hir Type<'hir>;

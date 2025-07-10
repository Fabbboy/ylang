use getset::Getters;
use sable_arena::interner::Interner;

use crate::item::DefId;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TypeKind {}

#[derive(Debug, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Type {
  id: DefId,
  kind: TypeKind,
}

pub type TypeId<'hir> = &'hir Type;
pub type TypeInterner<'hir> = Interner<'hir, Type>;

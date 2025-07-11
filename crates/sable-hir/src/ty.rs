use getset::Getters;
use sable_ast::types::Type as AstType;
use typed_builder::TypedBuilder;

use crate::{
  context::Context,
  module::DefId,
};

#[derive(Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path<'hir> {
  segments: &'hir [Symbol],
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
  #[getset(get = "pub")]
  id: DefId,
  #[getset(get = "pub")]
  kind: TypeKind<'hir>,
}

pub type TypeId<'hir> = &'hir Type<'hir>;

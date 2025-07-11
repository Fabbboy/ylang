use getset::Getters;
use sable_ast::types::Type as AstType;
use sable_common::interner::Interned;
use typed_builder::TypedBuilder;

use crate::{context::Context, module::DefId};

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

impl<'hir> TypeKind<'hir> {
  pub fn from_ast(ast_type: &AstType<'hir>, context: &mut Context<'hir>) -> Self {
    todo!()
  }
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

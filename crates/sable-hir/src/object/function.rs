use getset::Getters;
use sable_arena::interner::Symbol;
use typed_builder::TypedBuilder;

use crate::ty::TypeId;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HirParameter<'hir> {
  #[getset(get = "pub")]
  name: Symbol<'hir>,
  #[getset(get = "pub")]
  type_: TypeId<'hir>,
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HirFunction<'hir> {
  #[getset(get = "pub")]
  name: Symbol<'hir>,
  #[getset(get = "pub")]
  params: &'hir [HirParameter<'hir>],
  #[getset(get = "pub")]
  return_type: TypeId<'hir>,
}

pub type HirFunctionId<'hir> = &'hir HirFunction<'hir>;

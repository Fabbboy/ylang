use getset::Getters;
use sable_resolve::Symbol;
use typed_builder::TypedBuilder;

use crate::ty::TypeId;

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HirParameter<'hir> {
  #[getset(get = "pub")]
  name: Symbol,
  #[getset(get = "pub")]
  type_: TypeId<'hir>,
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HirFunction<'hir> {
  #[getset(get = "pub")]
  id: Symbol,
  #[getset(get = "pub")]
  params: &'hir [HirParameter<'hir>],
  #[getset(get = "pub")]
  return_type: TypeId<'hir>,
}

pub type HirFunctionId<'hir> = &'hir HirFunction<'hir>;

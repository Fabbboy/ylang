use getset::{
  Getters,
  MutGetters,
  Setters,
};

use typed_builder::TypedBuilder;

use crate::objects::function::HirFunction;

#[derive(Debug, TypedBuilder, Default, Getters, Setters)]
pub struct HirModule<'hir> {
  #[getset(get = "pub", set = "pub")]
  funcs: &'hir [&'hir HirFunction<'hir>],
}

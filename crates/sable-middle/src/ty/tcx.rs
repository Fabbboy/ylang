use std::{
  collections::HashMap,
  pin::Pin,
};

use bumpalo::Bump;
use getset::Getters;

use crate::ty::types::{
  Ty,
  Type,
};

#[derive(Getters)]
pub struct TypeContext<'tcx> {
  #[getset(get = "pub")]
  heap: Pin<Box<Bump>>,
  #[getset(get = "pub")]
  types: HashMap<Type, Ty<'tcx>>,
}

impl<'tcx> TypeContext<'tcx> {
  pub fn new() -> Self {
    let heap = Box::pin(Bump::new());

    Self {
      heap,
      types: HashMap::new(),
    }
  }
}

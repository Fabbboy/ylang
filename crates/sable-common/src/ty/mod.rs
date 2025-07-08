use sable_arena::interner::Interner;

use crate::ty::types::Type;

pub mod def;
pub mod resolution;
pub mod types;

pub type TypeInterner<'intern, const CHUNK_SIZE: usize = 4096> =
  Interner<'intern, Type, CHUNK_SIZE>;

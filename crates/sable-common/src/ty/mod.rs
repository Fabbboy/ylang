use sable_arena::interner::Interner;

use crate::ty::types::Type;

pub mod def;
pub mod resolution;
pub mod types;

pub type TypeInterner<'intern> = Interner<'intern, Type>;

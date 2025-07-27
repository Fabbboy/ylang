#![feature(allocator_api)]
#![feature(ptr_metadata)]

pub mod arena;
pub mod typed_arena;

pub use typed_arena::TypedArena;

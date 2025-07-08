#![feature(allocator_api)]

pub mod arena;
pub mod interner;
#[cfg(test)]
mod tests;

pub use arena::{Arena, DroplessArena, TypedArena};
pub type RawArena = Arena;

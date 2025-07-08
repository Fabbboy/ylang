#![feature(allocator_api)]

pub mod arena;
pub mod interner;
#[cfg(test)]
mod tests;

pub type Arena = arena::RawArena<4096>;

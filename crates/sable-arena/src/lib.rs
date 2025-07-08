#![no_std]

#[cfg(test)]
mod tests;
pub mod arena;

pub type Arena = arena::RawArena<4096>;

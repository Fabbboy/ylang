pub mod arena;
#[cfg(test)]
mod tests;
pub mod interner;

pub type Arena = arena::RawArena<4096>;

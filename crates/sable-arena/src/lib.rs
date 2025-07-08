pub mod arena;
#[cfg(test)]
mod tests;

pub type Arena = arena::RawArena<4096>;

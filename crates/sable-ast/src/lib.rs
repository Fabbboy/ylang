#![feature(allocator_api)]

pub mod ast;
pub mod expression;
pub mod located;
pub mod objects;
pub mod statement;
pub mod token;
pub mod types;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct NodeId(pub usize);
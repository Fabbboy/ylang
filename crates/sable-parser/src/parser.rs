use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self { Self { lexer } }
    pub fn parse(&mut self) -> ParserStatus { ParserStatus::Ok }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserStatus {
    Ok,
    OhNo,
}

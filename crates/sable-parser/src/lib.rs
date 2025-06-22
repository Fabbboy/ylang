pub mod location;
pub mod token;
pub mod lexer;
pub mod parser;

pub use location::Location;
pub use token::{TokenType, TokenData, Token};
pub use lexer::Lexer;
pub use parser::{Parser, ParserStatus};

#[cfg(test)]
mod tests {
    use super::*;
    use sable_common::SourceManager;

    #[test]
    fn lex_simple() {
        let mut manager = SourceManager::new();
        let src = manager.add_content("a = 1", "test.sable");
        let mut lexer = Lexer::new(src);
        let mut tokens: Vec<TokenType> = lexer.map(|t| t.kind).collect();
        tokens.push(TokenType::Eof);
        assert_eq!(tokens, [TokenType::Identifier, TokenType::Assign, TokenType::Integer, TokenType::Eof]);
    }
}

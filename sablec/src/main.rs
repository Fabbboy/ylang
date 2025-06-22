use bumpalo::Bump;
use sable_common::manager::Manager;
use sable_parser::{lexer::Lexer, token::TokenKind};

const SOURCE: &str = r#"
    let idio = 123;
"#;

fn main() {
  let bump = Bump::new();

  let mut manager = Manager::new();
  let source = manager.add_source(SOURCE, "test.sable", &bump);

  let mut lexer = Lexer::new(source);
  while let Some(token) = lexer.next() {
    println!("{:?}", token);
    if token.kind().clone() == TokenKind::Eof {
      break;
    }
  }
}

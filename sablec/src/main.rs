use bumpalo::Bump;
use sable_common::manager::Manager;
use sable_parser::{lexer::Lexer, token::TokenKind};

const SOURCE: &str = r#"
    let idio = 123;
"#;

fn main() {
  let bump = Bump::new();

  let mut manager = Manager::new();
  manager.add_source(SOURCE, "test.sable", &bump);
  let source = manager.sources().last().unwrap();

  let mut lexer = Lexer::new(source);
  while let Some(token) = lexer.next() {
    println!("{:?}", token);
    if token.kind().clone() == TokenKind::Eof {
      break;
    }
  }
}

use std::io;

use bumpalo::Bump;
use sable_common::manager::Manager;
use sable_parser::{
  lexer::Lexer,
  token::TokenKind,
};
use sable_report::writer::DiagnosticWriter;

const SOURCE: &str = r#"
   // this is a comment
    let idio = 123;
"#;

fn main() {
  let bump = Bump::new();

  let mut manager = Manager::new();
  let writer = DiagnosticWriter::new(&manager, &mut io::stdout());

  manager.add_source(SOURCE, "test.sable", &bump);
  let source = manager
    .sources()
    .get("test.sable")
    .expect("Source not found");

  let mut lexer = Lexer::new(source);
  while let Some(token) = lexer.next() {
    println!("{:?}", token);
    if token.kind().clone() == TokenKind::Eof {
      break;
    }
  }
}

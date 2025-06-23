use std::io;

use bumpalo::Bump;
use sable_common::manager::Manager;
use sable_parser::{
  lexer::Lexer,
  token::TokenKind,
};
use sable_report::{
  cache::Cache,
  writer::DiagnosticWriter,
};

const SOURCE: &str = r#"
   // this is a comment
    let idio = 123;
"#;

fn main() {
  let bump = Bump::new();

  let mut manager = Manager::new();
  let mut cache = Cache::new(&bump);

  let writer = DiagnosticWriter::new(&cache, &mut io::stdout());

  let source = manager.add_source(SOURCE, "test.sable", &bump);
  cache.add_file("test.sable", source);

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

use std::io;

use bumpalo::Bump;
use sable_ast::ast::Ast;
use sable_common::manager::Manager;
use sable_parser::{
  lexer::Lexer,
  parser::{
    ParseStatus,
    Parser,
  },
};
use sable_report::{
  cache::Cache,
  writer::DiagnosticWriter,
};

const SOURCE: &str = r#"
// this is a comment
let idio = 123; #  
"#;

fn main() {
  let bump = Bump::new();

  let mut manager = Manager::new();
  let mut cache = Cache::new(&bump);
  let source = manager.add_source(SOURCE, "test.sable", &bump);
  cache.add_file("test.sable", source.clone());

  let mut binding = io::stdout();
  let mut writer = DiagnosticWriter::new(&cache, &mut binding);

  let lexer = Lexer::new(source.clone());
  let mut ast = Ast::new();
  let mut parser = Parser::new(lexer, &mut ast);
  match parser.parse(&mut writer) {
    ParseStatus::Success => {
      println!("Parsing completed successfully.");
    }
    ParseStatus::Error => {
      eprintln!("Parsing encountered errors.");
    }
  }
}

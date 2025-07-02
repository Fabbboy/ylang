use std::io;

use bumpalo::Bump;
use sable_ast::ast::Ast;
use sable_common::{
  cache::AriadneCache,
  manager::Manager,
  writer::ReportWriter,
};
use sable_parser::{
  lexer::Lexer,
  parser::{
    ParseStatus,
    Parser,
  },
};

const SOURCE: &str = r#"
func main(argc i32, argv i32**) i32 {
123;
123;
}
"#;

fn main() {
  let bump = Bump::new();

  let mut manager = Manager::new();
  let mut cache = AriadneCache::new();
  let source = manager.add_source(SOURCE, "test.sable", &bump);
  cache.add_file(&source);

  let mut stdout = io::stdout();
  let mut writer = ReportWriter::new(&mut cache, &mut stdout);

  let lexer = Lexer::new(source.clone());
  let mut ast = Ast::new();
  let mut parser = Parser::new(lexer, &mut ast);
  match parser.parse(&mut writer) {
    ParseStatus::Success => {
      println!("{:#?}", ast);
    }
    ParseStatus::Error => {
      eprintln!("Parsing encountered errors.");
    }
  }
}

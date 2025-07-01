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
// intentionally leaving out return type to showcase that syncing logic is broken
//Error: Unexpected token: `Brace(false)`
//   ╭─[ test.sable:4:1 ]
//   │
// 4 │ }
//   │ ┬
//   │ ╰── Expected one of: Func, Eof
//───╯
// in theory sync should skip over that brace till th next function declaration or eof

func main(argv i32) {
}

// as you can see the eof is directly after the brace
// still parser chokes on it
// but after choking on the brace we successfully find and end parsing with an eof token
// so skipping and lexing logic is broken
"#;

fn main() {
  let bump = Bump::new();

  let mut manager = Manager::new();
  let mut cache = AriadneCache::new();
  let source = manager.add_source(SOURCE, "test.sable", &bump);
  cache.add_file(&source);

  let mut binding = io::stdout();
  let mut writer = ReportWriter::new(&mut cache, &mut binding);

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

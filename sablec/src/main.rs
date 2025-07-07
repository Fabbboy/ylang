use std::io;

use bumpalo::Bump;
use clap::Parser;
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
    Parser as SableParser,
  },
};

/// Sable compiler
#[derive(Parser, Debug)]
#[command(name = "sablec")]
#[command(about = "A compiler for the Sable programming language")]
#[command(version)]
struct Args {
  /// Input source file to compile
  #[arg(value_name = "FILE")]
  input: String,
}

fn main() {
  let args = Args::parse();
  let ast_bump = Bump::new();
  let file_bump = Bump::new();

  let mut manager = Manager::new();
  let mut cache = AriadneCache::new();

  let (source_code, filename) = {
    match std::fs::read_to_string(&args.input) {
      Ok(content) => (content, args.input.clone()),
      Err(e) => {
        eprintln!("Error reading file '{}': {}", args.input, e);
        std::process::exit(1);
      }
    }
  };

  let source = manager.add_source(&source_code, &filename, &file_bump);
  cache.add_file(&source);

  let mut stdout = io::stdout();
  let mut writer = ReportWriter::new(&mut cache, &mut stdout);

  let lexer = Lexer::new(source.clone());
  let mut ast = Ast::new(&ast_bump);
  let mut parser = SableParser::new(lexer, &mut ast);
  match parser.parse(&mut writer) {
    ParseStatus::Success => {
      println!("AST: {:#?}", ast);
    }
    ParseStatus::Error => {
      eprintln!("Parsing encountered errors.");
      std::process::exit(1);
    }
  }
}

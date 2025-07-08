use std::io;

use clap::Parser as ClapParser;

use sable_common::file::manager::Manager;
use sable_errors::{cache::ErrorCache, writer::ReportWriter};
use sable_middle::gctx::GlobalContext;
use sable_parse::{
  lexer::Lexer,
  parser::Parser,
};

/// Sable compiler
#[derive(ClapParser, Debug)]
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
  let mut manager = Manager::new();
  let mut cache = ErrorCache::new();

  let (source_code, filename) = {
    match std::fs::read_to_string(&args.input) {
      Ok(content) => (content, args.input.clone()),
      Err(e) => {
        eprintln!("Error reading file '{}': {}", args.input, e);
        std::process::exit(1);
      }
    }
  };

  let source = manager.add_source(&source_code, &filename);
  cache.add_file(&source);

  let mut stdout = io::stdout();
  let mut writer = ReportWriter::new(&mut cache, &mut stdout);

  let lexer = Lexer::new(source.clone());
  let mut parser = Parser::new(lexer);
  let ast = match parser.parse(&mut writer) {
    Ok(ast) => ast,
    Err(_) => {
      eprintln!("Parsing failed. See errors above.");
      std::process::exit(1);
    }
  };
  println!("AST: {:#?}", ast);
  let gctx = GlobalContext::new();
  println!("Global Context: {:#?}", gctx);
}

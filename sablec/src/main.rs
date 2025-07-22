use std::io;

use clap::Parser as ClapParser;
use sable_arena::arena::Arena;
use sable_common::file::manager::Manager;
use sable_errors::{
  cache::ErrorCache,
  writer::ReportWriter,
};
use sable_middle::tu::TranslationUnit;
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
  let file_arena = Arena::new();
  let middle_arena = Arena::new();

  let mut module = TranslationUnit::new(&middle_arena, 1);

  let args = Args::parse();
  let mut manager = Manager::new(&file_arena);
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

  let main_ast_arena = Arena::new();
  let mut main_ast = module.obtain(&main_ast_arena, 0).unwrap();

  let mut parser = Parser::new(lexer, &mut main_ast, &mut writer);
  match parser.parse() {
    Ok(_) => {
      println!(
        "Successfully parsed {} function(s).",
        main_ast.funcs().len()
      );
    }
    Err(_) => {
      eprintln!("Parsing failed. See errors above.");
      std::process::exit(1);
    }
  };

  println!("{:#?}", module);
}

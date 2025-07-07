use std::io;

use clap::Parser;
use sable_ast::ast::Ast;
use sable_common::{
  cache::AriadneCache,
  context::Context,
  manager::Manager,
  writer::ReportWriter,
};
use sable_hir::{
  lowering::AstLowering,
  module::HirModule,
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
  let ctx = Context::default();

  let mut manager = Manager::new(&ctx);
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

  let source = manager.add_source(&source_code, &filename);
  cache.add_file(&source);

  let mut stdout = io::stdout();
  let mut writer = ReportWriter::new(&mut cache, &mut stdout);

  let lexer = Lexer::new(source.clone());
  let mut ast = Ast::new(&ctx);
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

  let mut hir = HirModule::default();
  let mut lowering = AstLowering::new(&ast, &mut hir, &ctx);
  lowering.lower();
  println!("HIR: {:#?}", hir);
}

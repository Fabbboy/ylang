use std::io;

use clap::Parser as ClapParser;
use sable_arena::arena::Arena;
use sable_ast::ast::Ast;
use sable_common::{
  file::manager::Manager,
  writer::ReportWriter,
};
use sable_hir::package::Package;
use sable_lowering::ast_lower::lower::AstLowering;
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
  let hir_arena = Arena::new();

  let args = Args::parse();
  let mut manager = Manager::new(&file_arena);

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

  let mut stdout = io::stdout();
  let mut writer = ReportWriter::new(manager.error_cache_mut(), &mut stdout);

  let package: Package = {
    let main_ast_arena = Arena::new();
    let main_ast = Ast::new(&main_ast_arena);

    let mut asts = Vec::new();
    asts.push(main_ast);
    let mut main_ast = &mut asts[0];

    {
      let lexer = Lexer::new(source.clone());

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

      let pkg = Package::new(&hir_arena, &asts);
      let mut lowerer = AstLowering::new(&asts, &pkg, &mut writer);

      match lowerer.lower() {
        Ok(_) => {
          println!("Successfully lowered AST to HIR.");
        }
        Err(_) => {
          eprintln!("Lowering failed.");
          std::process::exit(1);
        }
      }

      pkg
    }
  };

  println!("{:#?}", package);
}

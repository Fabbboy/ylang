use std::io;

use clap::Parser as ClapParser;
use sable_arena::{
  TypedArena,
  arena::Arena,
};
use sable_ast::{
  ast::Ast,
  expression::Expression,
  objects::function::FunctionParam,
};
use sable_common::{
  file::{
    manager::Manager,
    source::Source,
  },
  interner::StrInterner,
  writer::ReportWriter,
};
use sable_hir::{
  hir::item::Item,
  package::Package,
};
use sable_lowering::ast_lower::resolver::Resolver;
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
  let file_arena: TypedArena<Source> = TypedArena::new();
  let item_arena: TypedArena<Item> = TypedArena::new();
  let str_intern_arena = Arena::new();
  let str_intern = StrInterner::new(&str_intern_arena);

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

  let mut package = Package::new(&item_arena, &str_intern);

  {
    let expr_arena: TypedArena<Expression> = TypedArena::new();
    let param_arena: TypedArena<FunctionParam> = TypedArena::new();
    let main_ast = Ast::new(&expr_arena, &param_arena);

    let mut asts = vec![main_ast];
    let main_ast = &mut asts[0];

    {
      let lexer = Lexer::new(source.clone());

      let mut parser = Parser::new(lexer, main_ast, &mut writer, &str_intern);
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

      let mut resolver = Resolver::new(&asts, &mut package, &mut writer);
      match resolver.resolve() {
        Ok(_) => println!("Resolution successful."),
        Err(_) => {
          eprintln!("Resolution failed. See errors above.");
          std::process::exit(1);
        }
      };
    }
  };

  println!("{:#?}", package);
}

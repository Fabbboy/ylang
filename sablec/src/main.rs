use std::io;

use clap::Parser as ClapParser;
use sable_arena::TypedArena;
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
  writer::ReportWriter,
};
use sable_hir::{
  hir::{
    item::Item,
    module::Module,
  },
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
  let file_arena: TypedArena<Source<'_>> = TypedArena::new();
  let module_arena: TypedArena<Module<'_>> = TypedArena::new();
  let item_arena: TypedArena<Item<'_>> = TypedArena::new();

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
    let expr_arena: TypedArena<Expression<'_>> = TypedArena::new();
    let param_arena: TypedArena<FunctionParam<'_>> = TypedArena::new();
    let main_ast = Ast::new(&expr_arena, &param_arena);

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

      let mut pkg = Package::new(&module_arena, &item_arena, &asts);

      let mut resolver = Resolver::new(&asts, &mut pkg, &mut writer);
      match resolver.resolve() {
        Ok(_) => println!("Resolution successful."),
        Err(_) => {
          eprintln!("Resolution failed. See errors above.");
          std::process::exit(1);
        }
      };

      pkg
    }
  };

  println!("{:#?}", package);
}

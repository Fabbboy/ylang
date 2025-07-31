#![feature(allocator_api)]
use std::{
  cell::OnceCell,
  io,
  sync::Arc,
};

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
    source::{
      self,
      Source,
    },
  },
  interner::StrInterner,
  writer::{
    ReportWriter,
    Sink,
  },
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
  input: Vec<String>,
}

struct ParseCtx<'ast> {
  expr_arena: TypedArena<Expression<'ast>>,
  param_arena: TypedArena<FunctionParam<'ast>>,
  ast: OnceCell<Ast<'ast>>,
}

fn parse_file<'src, 'ast, D>(
  source: Arc<Source<'src>, &'src TypedArena<Source<'src>>>,
  asts_arena: &'ast TypedArena<ParseCtx<'ast>>,
  str_intern: &'ast StrInterner<'src>,
  writer: &mut D,
) -> Result<&'ast ParseCtx<'ast>, ()>
where
  D: Sink<'src>,
  'src: 'ast,
{
  let ctx = asts_arena.alloc(ParseCtx {
    expr_arena: TypedArena::new(),
    param_arena: TypedArena::new(),
    ast: OnceCell::new(),
  });

  let _ = ctx.ast.set(Ast::new(&ctx.expr_arena, &ctx.param_arena));

  let lexer = Lexer::new(source.clone());
  let ast = ctx.ast.get_mut().unwrap();
  let mut parser = Parser::new(lexer, ast, writer, str_intern);
  match parser.parse() {
    Ok(_) => {
      println!("Successfully parsed {} function(s).", ast.funcs().len());
      Ok(&*ctx)
    }
    Err(_) => {
      eprintln!("Parsing failed. See errors above.");
      Err(())
    }
  }
}

fn main() {
  let file_arena: TypedArena<Source> = TypedArena::new();
  let asts_arena: TypedArena<ParseCtx> = TypedArena::new();
  let item_arena: TypedArena<Item> = TypedArena::new();
  let str_intern_arena = Arena::new();
  let str_intern = StrInterner::new(&str_intern_arena);

  let args = Args::parse();
  let mut manager = Manager::new(&file_arena);
  let mut package = Package::new(&item_arena, &str_intern);

  let mut sources = vec![];
  for filename in args.input {
    let source_code = match std::fs::read_to_string(&filename) {
      Ok(content) => content,
      Err(e) => {
        eprintln!("Error reading file '{}': {}", filename, e);
        std::process::exit(1);
      }
    };

    let src = manager.add_source(&source_code, &filename);
    sources.push(src);
  }

  let mut stdout = io::stdout();
  let mut writer = ReportWriter::new(manager.error_cache_mut(), &mut stdout);

  let mut ctxs = vec![];
  for source in sources {
    match parse_file(source, &asts_arena, &str_intern, &mut writer) {
      Ok(ctx) => ctxs.push(ctx),
      Err(_) => {
        eprintln!("Failed to parse source file.");
        std::process::exit(1);
      }
    }
  }
  /*
  let mut resolver = Resolver::new(&ast_refs, &mut package, &mut writer);
  match resolver.resolve() {
    Ok(_) => println!("Resolution successful."),
    Err(_) => {
      eprintln!("Resolution failed. See errors above.");
      std::process::exit(1);
    }
  }; */
  println!("{:#?}", package);
}

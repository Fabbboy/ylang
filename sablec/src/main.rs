#![feature(allocator_api)]
use clap::Parser as ClapParser;
use heaped::arena::dropless::DroplessArena;
//TODO: Remove sable-arena and replace with heaped arenas
// TODO: Use DroplessArena and TypedArena<T>
use sable_arena::{
  TypedArena,
  arena::Arena,
};
use std::{
  io,
  sync::Arc,
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
  writer::{
    ReportWriter,
    Sink,
  },
};
use sable_hir::{
  hir::item::Item,
  package::Package,
};
use sable_lowering::resolver::Resolver;
use sable_middle::{
  context::Context,
  scope::{
    Scope,
    Symbol,
  },
};
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

struct ParseCtx<'ast, 'src> {
  expr_arena: TypedArena<Expression<'ast, 'src>>,
  param_arena: TypedArena<FunctionParam<'src>>,
}

fn parse_file<'src, 'ast, D>(
  source: Arc<Source<'src>, &'src TypedArena<Source<'src>>>,
  asts_arena: &'ast TypedArena<Ast<'ast, 'src>>,
  ctx: &'ast ParseCtx<'ast, 'src>,
  str_intern: &'ast StrInterner<'src>,
  writer: &mut D,
) -> Result<&'ast mut Ast<'ast, 'src>, ()>
where
  D: Sink<'src>,
  'src: 'ast,
{
  let ast = asts_arena.alloc(Ast::new(&ctx.expr_arena, &ctx.param_arena));

  let lexer = Lexer::new(source.clone());
  let mut parser = Parser::new(lexer, ast, writer, str_intern);
  match parser.parse() {
    Ok(_) => {
      println!("Successfully parsed {} function(s).", ast.funcs().len());
      Ok(ast)
    }
    Err(_) => {
      eprintln!("Parsing failed. See errors above.");
      Err(())
    }
  }
}

fn resolve_asts<'ast, 'src, 'resolve>(
  asts: &'resolve mut [&'ast mut Ast<'ast, 'src>],
  context: &'resolve mut Context<'resolve, 'src>,
) -> Result<(), ()> {
  let mut resolver = Resolver::new(asts, context);
  resolver.resolve()
}

fn main() {
  let args = Args::parse();

  let str_intern_arena = DroplessArena::new(4096); // DO NOT REMOVE WE ARE REPLACEING SABLE ARENA WITH HEAPED ARENA
  let str_intern = StrInterner::new(&str_intern_arena);

  let file_arena: TypedArena<Source> = TypedArena::new(); // outlives everything 
  let mut manager = Manager::new(&file_arena); // outlives everything 

  let scope_arena: TypedArena<Scope> = TypedArena::new();
  let symbol_arena: TypedArena<Symbol> = TypedArena::new();
  let mut context = Context::new(&str_intern, &scope_arena, &symbol_arena);
  let item_arena: TypedArena<Item> = TypedArena::new();
  let package = Package::new(&item_arena);

  let mut sources = vec![];
  let mut ctxs = vec![];
  let asts_arena = TypedArena::new();

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
    let ctx = ParseCtx {
      expr_arena: TypedArena::new(),
      param_arena: TypedArena::new(),
    };
    ctxs.push(ctx);
  }

  let mut stdout = io::stdout();
  let mut writer = ReportWriter::new(manager.error_cache_mut(), &mut stdout);

  let mut asts = vec![];
  for (source, ctx) in sources.iter().zip(ctxs.iter_mut()) {
    match parse_file(source.clone(), &asts_arena, ctx, &str_intern, &mut writer) {
      Ok(ast) => {
        asts.push(ast);
      }
      Err(_) => {
        eprintln!("Failed to parse file '{}'.", source.filename());
      }
    }
  }

  resolve_asts(&mut asts, &mut context).unwrap_or_else(|_| {
    eprintln!("Failed to resolve ASTs.");
    std::process::exit(1);
  });

  println!("{:#?}", asts);
  println!("{:#?}", package);
}

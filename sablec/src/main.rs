#![feature(allocator_api)]
use clap::Parser as ClapParser;
use sable_arena::{
  TypedArena,
  arena::Arena,
};
use sable_lowering::scope::Symbol;
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
use sable_lowering::{
  resolver::Resolver,
  scope::Scope,
};
use sable_middle::context::Context;
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
}

fn parse_file<'src, 'ast, D>(
  source: Arc<Source<'src>, &'src TypedArena<Source<'src>>>,
  asts_arena: &'ast TypedArena<Ast<'ast>>,
  ctx: &'ast ParseCtx<'ast>,
  str_intern: &'ast StrInterner<'src>,
  writer: &mut D,
) -> Result<&'ast mut Ast<'ast>, ()>
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

fn main() {
  let file_arena: TypedArena<Source> = TypedArena::new();
  let asts_arena: TypedArena<Ast> = TypedArena::new();
  let item_arena: TypedArena<Item> = TypedArena::new();
  let str_intern_arena = Arena::new();
  let str_intern = StrInterner::new(&str_intern_arena);
  let scope_arena: TypedArena<Scope> = TypedArena::new();
  let symbol_arena: TypedArena<Symbol> = TypedArena::new();

  let args = Args::parse();
  let mut manager = Manager::new(&file_arena);
  let context = Context::new(&str_intern, &scope_arena, &symbol_arena);
  let package = Package::new(&item_arena);

  let mut ctxs = vec![];
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

  let mut resolver = Resolver::new(&mut asts);
  match resolver.resolve() {
    Ok(_) => {
      println!("Successfully resolved all ASTs.");
    }
    Err(_) => {
      eprintln!("Error resolving ASTs.");
      std::process::exit(1);
    }
  }

  println!("{:#?}", asts);
  println!("{:#?}", package);
}

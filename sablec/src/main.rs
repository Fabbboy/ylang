use std::{
  io,
  sync::Arc,
};

use clap::Parser as ClapParser;
use sable_arena::{
  arena::Arena,
  interner::{
    StrInterner,
    Symbol,
  },
};
use sable_common::{
  file::manager::Manager,
  location::Location,
};
use sable_errors::{
  cache::ErrorCache,
  writer::ReportWriter,
};
use sable_hir::{
  item::{
    DefId,
    Item,
  },
  module::{
    ModId,
    Module,
  },
  object::function::HirFunction,
  ty::{
    Type,
    TypeInterner,
    TypeKind,
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
  input: String,
}

fn main() {
  let file_arena = Arena::new();
  let ast_arena = Arena::new();
  let hir_arena = Arena::new();

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
  let mut parser = Parser::new(lexer);
  let ast = match parser.parse(&mut writer, &ast_arena) {
    Ok(ast) => ast,
    Err(_) => {
      eprintln!("Parsing failed. See errors above.");
      std::process::exit(1);
    }
  };

  println!("AST: {:#?}", ast);

  let mut str_interner = StrInterner::new(&hir_arena);
  let mut type_interner = TypeInterner::new(&hir_arena);

  let default_type = Type::builder()
    .id(DefId(ModId(0), 0))
    .kind(TypeKind::None)
    .build();
  let interned_default_type = type_interner.intern(&default_type);

  let hir_func = HirFunction::builder()
    .name(Symbol(str_interner.intern("example_function")))
    .params(&[])
    .return_type(interned_default_type)
    .build();

  let hir_func_id = &hir_arena.alloc(hir_func);
  let def_id = DefId(ModId(0), 1);
  let item = Item::builder()
    .id(def_id)
    .kind(sable_hir::item::ItemKind::Func(hir_func_id))
    .location(Location::new(0..0, Arc::from("lol")))
    .build();
  let item_id = hir_arena.alloc(item);
  let items: [&Item; 1] = [item_id];

  let module = Module::builder()
    .id(ModId(0))
    .items(&items)
    .arena(&hir_arena)
    .build();

  println!("Module: {:#?}", module);
}

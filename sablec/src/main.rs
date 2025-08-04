#![feature(allocator_api)]
use std::{
  io,
  sync::Arc,
};

use clap::Parser as ClapParser;
use sable_arena::{
  arena::Arena,
  TypedArena,
};
use sable_ast::{
  ast::Ast,
  expression::{
    assign_expression::AssignExpression,
    binary_expression::BinaryExpression,
    block_expression::BlockExpression,
    identifier_expression::IdentifierExpression,
    literal_expression::LiteralExpression,
    Expression,
    ExpressionVisitorMut,
  },
  objects::function::FunctionParam,
  statement::{
    variable_statement::VariableStatement,
    Statement,
    StatementVisitorMut,
  },
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

/// Visitor that counts the number of expressions in an AST.
struct ExprCounter {
  count: usize,
}

impl ExprCounter {
  fn new() -> Self {
    Self { count: 0 }
  }
}

impl<'ast> ExpressionVisitorMut<'ast> for ExprCounter {
  type VisitReturn = ();

  fn visit_block_mut(
    &mut self,
    block: &mut BlockExpression<'ast>,
    _expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    self.count += 1;
    for stmt in block.body_mut() {
      StatementVisitorMut::visit_stmt_mut(self, stmt);
    }
  }

  fn visit_literal_mut(
    &mut self,
    _literal: &mut LiteralExpression,
    _expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    self.count += 1;
  }

  fn visit_assign_mut(
    &mut self,
    assign: &mut AssignExpression<'ast>,
    _expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    self.count += 1;
    ExpressionVisitorMut::visit_expr_mut(self, assign.value_mut());
  }

  fn visit_binary_mut(
    &mut self,
    binary: &mut BinaryExpression<'ast>,
    _expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    self.count += 1;
    match binary {
      BinaryExpression::Add(expr) => {
        ExpressionVisitorMut::visit_expr_mut(self, expr.left_mut());
        ExpressionVisitorMut::visit_expr_mut(self, expr.right_mut());
      }
      BinaryExpression::Subtract(expr) => {
        ExpressionVisitorMut::visit_expr_mut(self, expr.left_mut());
        ExpressionVisitorMut::visit_expr_mut(self, expr.right_mut());
      }
      BinaryExpression::Multiply(expr) => {
        ExpressionVisitorMut::visit_expr_mut(self, expr.left_mut());
        ExpressionVisitorMut::visit_expr_mut(self, expr.right_mut());
      }
      BinaryExpression::Divide(expr) => {
        ExpressionVisitorMut::visit_expr_mut(self, expr.left_mut());
        ExpressionVisitorMut::visit_expr_mut(self, expr.right_mut());
      }
    }
  }

  fn visit_identifier_mut(
    &mut self,
    _identifier: &mut IdentifierExpression,
    _expr: &mut Expression<'ast>,
  ) -> Self::VisitReturn {
    self.count += 1;
  }
}

impl<'ast> StatementVisitorMut<'ast> for ExprCounter {
  type VisitReturn = ();

  fn visit_expression_mut(
    &mut self,
    expr: &mut Expression<'ast>,
    _statement: &mut Statement<'ast>,
  ) -> Self::VisitReturn {
    ExpressionVisitorMut::visit_expr_mut(self, expr);
  }

  fn visit_variable_mut(
    &mut self,
    variable: &mut VariableStatement<'ast>,
    _statement: &mut Statement<'ast>,
  ) -> Self::VisitReturn {
    ExpressionVisitorMut::visit_expr_mut(self, variable.initializer_mut());
  }
}

fn main() {
  let file_arena: TypedArena<Source> = TypedArena::new();
  let asts_arena: TypedArena<Ast> = TypedArena::new();
  let item_arena: TypedArena<Item> = TypedArena::new();
  let str_intern_arena = Arena::new();
  let str_intern = StrInterner::new(&str_intern_arena);

  let args = Args::parse();
  let mut manager = Manager::new(&file_arena);
  let package = Package::new(&item_arena, &str_intern);

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

  // Count expressions across all parsed ASTs using the mutable visitor.
  let mut counter = ExprCounter::new();
  for ast in &mut asts {
    for func in ast.funcs_mut() {
      if let Some(block) = func.block_mut() {
        for stmt in block.body_mut() {
          StatementVisitorMut::visit_stmt_mut(&mut counter, stmt);
        }
      }
    }
  }
  println!("Total expressions: {}", counter.count);

  /*let mut resolver = Resolver::new(&mut asts, &mut package, &mut writer);
  match resolver.resolve() {
    Ok(_) => println!("Resolution successful."),
    Err(_) => {
      eprintln!("Resolution failed. See errors above.");
      std::process::exit(1);
    }
  };
  */
  println!("{:#?}", asts);
  println!("{:#?}", package);
}

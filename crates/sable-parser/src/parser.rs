use sable_ast::ast::Ast;
use sable_report::sink::Sink;

use crate::lexer::Lexer;

pub enum ParseStatus {
  Success,
  Error,
}

pub struct Parser<'ctx, 'p, D> {
  engine: &'p D,
  lexer: Lexer<'ctx>,
  ast: &'p mut Ast,
}

impl<'ctx, 'p, D> Parser<'ctx, 'p, D>
where
  D: Sink,
{
  pub fn new(engine: &'p D, lexer: Lexer<'ctx>, ast: &'p mut Ast) -> Self {
    Self { engine, lexer, ast }
  }

  pub fn parse(&mut self) -> ParseStatus {
    let status = ParseStatus::Success;

    status
  }
}

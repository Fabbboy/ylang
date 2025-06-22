use sable_common::source::Source;

use crate::token::Token;

pub struct Lexer<'ctx> {
  source: &'ctx Source<'ctx>,
  raw: &'ctx str,

  pos: usize,
  start: usize,

  next: Token<'ctx>,
}

impl<'ctx> Lexer<'ctx> {
  pub fn new(source: &'ctx Source<'ctx>) -> Self {
    Self {
      raw: source.content().as_str(),
      source,

      pos: 0,
      start: 0,

      next: Token::default(),
    }
  }

  fn lex(&mut self) -> Token<'ctx> {
    Token::default()
  }
}

impl<'ctx> Iterator for Lexer<'ctx> {
  type Item = Token<'ctx>;

  fn next(&mut self) -> Option<Self::Item> {
    let cache = self.next.clone();
    self.next = self.lex();
    return Some(cache);

  }
}

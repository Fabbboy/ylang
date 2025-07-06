use sable_ast::ast::Ast;

pub struct TypeSimChecker<'c, 'ctx> {
  ast: &'c Ast<'ctx>,
}

impl<'c, 'ctx> TypeSimChecker<'c, 'ctx> {
  pub fn new(ast: &'c Ast<'ctx>) -> Self {
    Self { ast }
  }
}

use sable_ast::ast::Ast;

use crate::type_sim_checker::TypeSimChecker;

pub struct SemanticAnalyzer<'s, 'ctx> {
  ast: &'s Ast<'ctx>,
  type_sim: TypeSimChecker<'s, 'ctx>,
}

impl<'s, 'ctx> SemanticAnalyzer<'s, 'ctx> {
  pub fn new(ast: &'s Ast<'ctx>) -> Self {
    let type_sim = TypeSimChecker::new(ast);
    Self { ast, type_sim }
  }
}

use sable_ast::{
  expression::VisitExpression,
  statement::VisitStatement,
};
use sable_hir::package::Package;

pub trait Analyzer<'ast, 'sema>
where
  Self: VisitStatement<'ast> + VisitExpression<'ast>,
{
  fn new(package: &mut Package<'ast, 'sema>) -> Self;
}

use crate::expression::HirExpr;

#[derive(Debug)]
pub enum HirStatement<'hir> {
  Expression(HirExpr<'hir>),
}

pub type HirStmt<'hir> = &'hir HirStatement<'hir>;

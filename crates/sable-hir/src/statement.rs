use crate::expression::HirExpresison;

#[derive(Debug)]
pub enum HirStatement {
  Expression(HirExpresison),
}

pub mod assign_expression;
pub mod binary_expression;
pub mod block_expression;
pub mod identifier_expression;
pub mod literal_expression;

pub use assign_expression::AssignExpression;
pub use binary_expression::BinaryExpression;
pub use block_expression::BlockExpression;
pub use identifier_expression::IdentifierExpression;
pub use literal_expression::LiteralExpression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expression<'ctx> {
  Block(BlockExpression<'ctx>),
  Literal(LiteralExpression),
  Assign(AssignExpression<'ctx>),
  Binary(BinaryExpression<'ctx>),
  Identifier(IdentifierExpression),
}

impl<'ctx> Expression<'ctx> {
  pub fn location(&self) -> &crate::location::Location {
    match self {
      Expression::Block(block) => block.location(),
      Expression::Literal(literal) => literal.location(),
      Expression::Assign(assign) => assign.location(),
      Expression::Binary(binary) => binary.location(),
      Expression::Identifier(identifier) => identifier.location(),
    }
  }
}

pub trait VisitExpression<'ctx> {
  fn visit_block<T>(&mut self, block: &BlockExpression<'ctx>) -> T;
  fn visit_literal<T>(&mut self, literal: &LiteralExpression) -> T;
  fn visit_assign<T>(&mut self, assign: &AssignExpression<'ctx>) -> T;
  fn visit_binary<T>(&mut self, binary: &BinaryExpression<'ctx>) -> T;
  fn visit_identifier<T>(&mut self, identifier: &IdentifierExpression) -> T;

  fn visit_expression<T>(&mut self, expression: &Expression<'ctx>) -> T {
    match expression {
      Expression::Block(block) => self.visit_block(block),
      Expression::Literal(literal) => self.visit_literal(literal),
      Expression::Assign(assign) => self.visit_assign(assign),
      Expression::Binary(binary) => self.visit_binary(binary),
      Expression::Identifier(identifier) => self.visit_identifier(identifier),
    }
  }
}

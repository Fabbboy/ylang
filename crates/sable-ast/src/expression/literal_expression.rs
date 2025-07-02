#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum LiteralExpression {
  Integer(i64),
  Float(f64),
}

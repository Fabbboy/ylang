use getset::Getters;
use typed_builder::TypedBuilder;


#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum LiteralExpression {
  Integer(IntegerExpression),
  Float(FloatExpression),
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IntegerExpression  {
  #[getset(get = "pub")]
  value: i64,
}

#[derive(Debug, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FloatExpression {
  #[getset(get = "pub")]
  value: f64,
}

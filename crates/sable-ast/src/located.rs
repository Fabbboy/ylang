use getset::Getters;
use sable_common::location::Location;
use typed_builder::TypedBuilder;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Located<'loc, T> {
  #[getset(get = "pub")]
  value: T,
  #[getset(get = "pub")]
  location: Location<'loc>,
}


impl<'loc, T> From<Located<'loc, T>> for Located<'loc, Box<T>> {
  fn from(located: Located<'loc, T>) -> Self {
    Self {
      value: Box::new(located.value),
      location: located.location,
    }
  }
}

impl<'loc, T> Located<'loc, T> {
  pub fn replace(&self, value: T) -> Self {
    Self {
      value,
      location: self.location.clone(),
    }
  }
}

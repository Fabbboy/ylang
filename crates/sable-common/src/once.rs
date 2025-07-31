#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Once<T> {
  #[default]
  Uninit,
  Init(T),
}

impl<T> Once<T> {
  pub fn new_uninit() -> Self {
    Once::Uninit
  }

  pub fn new_init(value: T) -> Self {
    Once::Init(value)
  }

  pub fn init(&mut self, value: T) -> Result<(), T> {
    match self {
      Once::Uninit => {
        *self = Once::Init(value);
        Ok(())
      }
      Once::Init(_) => Err(value),
    }
  }

  pub fn get(&self) -> Option<&T> {
    match self {
      Once::Uninit => None,
      Once::Init(value) => Some(value),
    }
  }
}

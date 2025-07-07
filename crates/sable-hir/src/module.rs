use bumpalo::Bump;

pub struct HirModule {
  bump: Bump,
}

impl HirModule {
  pub fn new() -> Self {
    Self { bump: Bump::new() }
  }
}

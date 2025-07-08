use bumpalo::Bump;
use getset::Getters;

#[derive(Getters, Default, Debug)]
pub struct Context {
  #[getset(get = "pub")]
  master_bump: Bump,
}

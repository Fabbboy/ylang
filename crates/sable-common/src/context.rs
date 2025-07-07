use bumpalo::Bump;
use getset::Getters;

#[derive(Getters, Default, Debug)]
pub struct Context {
  #[getset(get = "pub")]
  master_bump: Bump,
  #[getset(get = "pub")]
  file_bump: Bump,
  #[getset(get = "pub")]
  ast_bump: Bump,
  #[getset(get = "pub")]
  hir_bump: Bump,
}

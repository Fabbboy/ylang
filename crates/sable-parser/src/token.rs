use getset::Getters;
use sable_ast::location::Location;

#[derive(Default, Clone, Debug, Eq)]
pub enum TokenKind {
  #[default]
  Eof,
}

impl PartialEq for TokenKind {
  fn eq(&self, other: &Self) -> bool {
    let descriminated_left = std::mem::discriminant(self);
    let descriminated_right = std::mem::discriminant(other);
    descriminated_left == descriminated_right
  }
}

#[derive(Getters, Default, Clone, Debug)]

pub struct Token<'ctx> {
  #[getset(get = "pub")]
  kind: TokenKind,
  #[getset(get = "pub")]
  lexeme: &'ctx str,
  #[getset(get = "pub")]
  location: Location<'ctx>,
}

impl<'ctx> Token<'ctx> {
  pub fn new(kind: TokenKind, lexeme: &'ctx str, location: Location<'ctx>) -> Self {
    Self {
      kind,
      lexeme,
      location,
    }
  }
}

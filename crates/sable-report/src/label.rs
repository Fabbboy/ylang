use std::{
  fmt::Display,
  io,
};

use colored::Colorize;
use getset::Getters;
use pretty::RcDoc;
use typed_builder::TypedBuilder;

use crate::{
  cache::Cache,
  span::Span,
};

pub enum LabelKind {
  Note,
}

impl Display for LabelKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LabelKind::Note => write!(f, "{}", "Note".blue()),
    }
  }
}

#[derive(Getters, TypedBuilder)]
pub struct Label<'ctx> {
  #[getset(get = "pub")]
  kind: LabelKind,
  #[getset(get = "pub")]
  #[builder(default, setter(into))]
  message: Option<&'ctx str>,
  #[getset(get = "pub")]
  #[builder(default)]
  code: Option<Span<'ctx>>,
}

impl<'ctx> Label<'ctx> {
  pub fn to_doc(&self, cache: &Cache<'ctx>) -> RcDoc<'ctx> {
    let mut doc = RcDoc::text(format!("{}: {}", self.kind, self.message.unwrap_or(" ")));
    if let Some(code) = &self.code {
      doc = doc.append(RcDoc::line()).append(code.to_doc(cache));
    }
    doc
  }

  pub fn write(&self, cache: &Cache<'ctx>, out: &mut dyn io::Write) -> io::Result<()> {
    self.to_doc(cache).render(100, out)?;
    writeln!(out)?;
    Ok(())
  }
}

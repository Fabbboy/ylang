use std::fmt::Display;

use colored::Colorize;
use pretty::RcDoc;
use getset::Getters;
use typed_builder::TypedBuilder;

use crate::{
  cache::Cache,
  span::Span,
  label::Label,
};

pub enum DiagnosticLevel {
  Error,
  Warning,
  Info,
}

impl Display for DiagnosticLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DiagnosticLevel::Error => write!(f, "{}", "Error".red()),
      DiagnosticLevel::Warning => write!(f, "{}", "Warning".yellow()),
      DiagnosticLevel::Info => write!(f, "{}", "Info".blue()),
    }
  }
}

#[derive(Getters, TypedBuilder)]
pub struct Diagnostic<'ctx> {
  #[getset(get = "pub")]
  level: DiagnosticLevel,
  #[getset(get = "pub")]
  #[builder(default, setter(into))]
  message: Option<&'ctx str>,
  #[getset(get = "pub")]
  #[builder(default)]
  code: Option<Span<'ctx>>,
  #[builder(default, setter(into))]
  labels: Vec<Label<'ctx>>,
}

impl<'ctx> Diagnostic<'ctx> {
  pub fn write(&self, cache: &Cache<'ctx>, out: &mut dyn std::io::Write) -> std::io::Result<()> {
    let mut doc = RcDoc::text(format!("{}: {}", self.level, self.message.unwrap_or(" ")));
    if let Some(code) = &self.code {
      doc = doc.append(RcDoc::line()).append(code.to_doc(cache));
    }
    for label in &self.labels {
      doc = doc.append(RcDoc::line()).append(label.to_doc(cache));
    }

    doc.render(100, out)?;
    writeln!(out)?;
    Ok(())
  }
}

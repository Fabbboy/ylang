use std::fmt::Display;

use colored::Colorize;
use getset::Getters;
use typed_builder::TypedBuilder;

use crate::{
  cache::Cache,
  span::Span,
};

pub enum DiagnosticLevel {
  Error,
  Warning,
  Info,
}

impl Display for DiagnosticLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DiagnosticLevel::Error => write!(f, "{}", "error".red()),
      DiagnosticLevel::Warning => write!(f, "{}", "warning".yellow()),
      DiagnosticLevel::Info => write!(f, "{}", "info".blue()),
    }
  }
}

#[derive(Getters, TypedBuilder)]
pub struct Diagnostic<'ctx> {
  #[getset(get = "pub")]
  level: DiagnosticLevel,
  #[getset(get = "pub")]
  message: Option<&'ctx str>,
  #[getset(get = "pub")]
  code: Option<Span<'ctx>>,
}

impl<'ctx> Diagnostic<'ctx> {
  pub fn write(&self, cache: &Cache<'ctx>, out: &mut dyn std::io::Write) -> std::io::Result<()> {
    writeln!(out, "{}: {}", self.level, self.message.unwrap_or(" "))?;
    if let Some(code) = &self.code {
      code.write(cache, out)?;
    }

    Ok(())
  }
}

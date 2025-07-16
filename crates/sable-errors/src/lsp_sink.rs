use std::collections::HashMap;

use lsp_types::{
  Diagnostic as LspDiagnostic,
  DiagnosticSeverity,
  NumberOrString,
  Position,
  Range,
};

use crate::{
  diagnostic::{
    Diagnostic,
    Severity,
  },
  sink::Sink,
};
use sable_common::file::{
  FileId,
  Span,
};

pub struct LspSink<'ctx> {
  diagnostics: HashMap<FileId<'ctx>, Vec<LspDiagnostic>>,
}

impl<'ctx> LspSink<'ctx> {
  pub fn new() -> Self {
    Self {
      diagnostics: HashMap::new(),
    }
  }

  pub fn diagnostics(&self) -> &HashMap<FileId<'ctx>, Vec<LspDiagnostic>> {
    &self.diagnostics
  }

  fn to_lsp_range(span: &Span<'ctx>) -> Range {
    let (_file, r) = span;
    Range {
      start: Position {
        line: 0,
        character: r.start as u32,
      },
      end: Position {
        line: 0,
        character: r.end as u32,
      },
    }
  }

  fn to_lsp_severity(sev: Severity) -> DiagnosticSeverity {
    match sev {
      Severity::Error => DiagnosticSeverity::ERROR,
      Severity::Warning => DiagnosticSeverity::WARNING,
      Severity::Info => DiagnosticSeverity::INFORMATION,
      Severity::Hint => DiagnosticSeverity::HINT,
    }
  }
}

impl<'ctx> Sink<'ctx> for LspSink<'ctx> {
  type Error = ();

  fn emit(&mut self, diagnostic: Diagnostic<'ctx>) -> Result<(), Self::Error> {
    let (file, _) = diagnostic.range;
    let diag = LspDiagnostic {
      range: Self::to_lsp_range(&diagnostic.range),
      severity: Some(Self::to_lsp_severity(diagnostic.severity)),
      code: diagnostic.code.map(NumberOrString::String),
      source: diagnostic.source.clone(),
      message: diagnostic.message.clone(),
      ..Default::default()
    };
    self.diagnostics.entry(file).or_default().push(diag);
    Ok(())
  }
}

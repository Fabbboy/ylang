use std::io::Write;

use ariadne::{Report, ReportKind, Label as AriLabel, sources};

use sable_common::SourceManager;

use crate::cache::Cache;
use crate::diagnostic::{Diagnostic, Severity};

pub trait DiagnosticEngine {
    fn report(&mut self, diag: &Diagnostic);
}

pub struct StreamWriter<'a, W: Write> {
    writer: &'a mut W,
    cache: Cache<'a>,
}

impl<'a, W: Write> StreamWriter<'a, W> {
    pub fn new(writer: &'a mut W, manager: &'a SourceManager) -> Self {
        Self { writer, cache: Cache::new(manager) }
    }
}

impl<'a, W: Write> DiagnosticEngine for StreamWriter<'a, W> {
    fn report(&mut self, diag: &Diagnostic) {
        let kind = match diag.severity {
            Severity::Info => ReportKind::Advice,
            Severity::Warning => ReportKind::Warning,
            Severity::Error => ReportKind::Error,
        };
        let (file, range) = diag
            .code
            .as_ref()
            .map(|s| (s.source.clone(), s.range.clone()))
            .unwrap_or((String::from("<unknown>"), 0..0));
        let mut builder = Report::build(kind, (file.clone(), range.clone()));
        if let Some(msg) = &diag.message {
            builder = builder.with_message(msg.clone());
        }
        if let Some(code) = &diag.code {
            builder = builder.with_label(AriLabel::new((code.source.clone(), code.range.clone())));
        }
        for label in &diag.labels {
            let mut l = AriLabel::new((label.span.source.clone(), label.span.range.clone()));
            if let Some(m) = &label.message { l = l.with_message(m.clone()); }
            builder = builder.with_label(l);
        }
        if let Some(note) = &diag.note {
            builder = builder.with_note(note.clone());
        }
        let report = builder.finish();
        let mut srcs = Vec::new();
        if let Some(entry) = self.cache.get_entry(&file) {
            srcs.push((file.clone(), entry.source.content.clone()));
        }
        for label in &diag.labels {
            if srcs.iter().any(|(n, _)| n == &label.span.source) { continue; }
            if let Some(entry) = self.cache.get_entry(&label.span.source) {
                srcs.push((label.span.source.clone(), entry.source.content.clone()));
            }
        }
        if srcs.is_empty() {
            report.write(sources(vec![(file.clone(), String::new())]), &mut self.writer).ok();
        } else {
            report.write(sources(srcs), &mut self.writer).ok();
        }
    }
}

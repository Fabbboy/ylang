use sable_common::{Manager, RangeUsize, Source};
use std::collections::HashMap;

use std::io::{self, Write};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Line {
    pub range: RangeUsize,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub source: Rc<Source>,
    pub lines: Vec<Line>,
}

impl CacheEntry {
    pub fn new(source: Rc<Source>) -> Self {
        let mut lines = Vec::new();
        let content = &source.content;
        let mut start = 0;
        for (idx, ch) in content.char_indices() {
            if ch == '\n' {
                lines.push(Line { range: start..idx, line_number: lines.len() + 1 });
                start = idx + 1;
            }
        }
        if start < content.len() {
            lines.push(Line { range: start..content.len(), line_number: lines.len() + 1 });
        }
        Self { source, lines }
    }

    pub fn lines_for(&self, range: RangeUsize) -> &[Line] {
        let mut first = None;
        let mut last = None;
        for (i, line) in self.lines.iter().enumerate() {
            if line.range.start <= range.end && line.range.end > range.start {
                if first.is_none() { first = Some(i); }
                last = Some(i);
            }
        }
        match (first, last) {
            (Some(f), Some(l)) => &self.lines[f..=l],
            _ => &[],
        }
    }
}

pub struct Cache<'a> {
    pub manager: &'a Manager,
    entries: HashMap<String, CacheEntry>,
}

impl<'a> Cache<'a> {
    pub fn new(manager: &'a Manager) -> Self { Self { manager, entries: HashMap::new() } }

    pub fn add_entry(&mut self, source: Rc<Source>) {
        self.entries.entry(source.filename.clone()).or_insert_with(|| CacheEntry::new(source));
    }

    pub fn get_entry(&self, name: &str) -> Option<&CacheEntry> { self.entries.get(name) }
}

#[derive(Debug, Clone)]
pub struct Span {
    pub source: String,
    pub range: RangeUsize,
}

impl Span {
    pub fn new(source: String, range: RangeUsize) -> Self { Self { source, range } }
}

#[derive(Debug, Clone)]
pub struct Label {
    pub span: Span,
    pub message: Option<String>,
}

impl Label {
    pub fn new(span: Span) -> Self { Self { span, message: None } }
    pub fn with_message(mut self, msg: impl Into<String>) -> Self { self.message = Some(msg.into()); self }
}

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: Option<String>,
    pub code: Option<Span>,
    pub labels: Vec<Label>,
    pub note: Option<String>,
}

impl Diagnostic {
    pub fn new(severity: Severity) -> Self {
        Self { severity, message: None, code: None, labels: Vec::new(), note: None }
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self { self.message = Some(msg.into()); self }
    pub fn with_note(mut self, note: impl Into<String>) -> Self { self.note = Some(note.into()); self }
    pub fn with_code(mut self, span: Span) -> Self { self.code = Some(span); self }
    pub fn with_label(mut self, label: Label) -> Self { self.labels.push(label); self }
}

pub trait DiagnosticEngine {
    fn report(&mut self, diag: &Diagnostic);
}

pub struct StreamWriter<'a, W: Write> {
    writer: &'a mut W,
    cache: &'a Cache<'a>,
}

impl<'a, W: Write> StreamWriter<'a, W> {
    pub fn new(writer: &'a mut W, cache: &'a Cache<'a>) -> Self { Self { writer, cache } }
}

impl<'a, W: Write> DiagnosticEngine for StreamWriter<'a, W> {
    fn report(&mut self, diag: &Diagnostic) {
        let _ = write!(self.writer, "{:?}: ", diag.severity);
        if let Some(msg) = &diag.message {
            let _ = writeln!(self.writer, "{}", msg);
        } else {
            let _ = self.writer.write_all(b"\n");
        }
        if let Some(code) = &diag.code {
            write_span(&mut *self.writer, code, self.cache).ok();
        }
        for label in &diag.labels {
            write!(self.writer, "[Note]: ").ok();
            if let Some(msg) = &label.message {
                writeln!(self.writer, "{}", msg).ok();
            }
            write_span(&mut *self.writer, &label.span, self.cache).ok();
        }
    }
}

fn write_span<W: Write>(mut w: W, span: &Span, cache: &Cache<'_>) -> io::Result<()> {
    if let Some(entry) = cache.get_entry(&span.source) {
        if let Some(line) = entry.lines_for(span.range.clone()).first() {
            let column = span.range.start - line.range.start + 1;
            writeln!(w, "[{}:{}:{}]", span.source, line.line_number, column)?;
            for l in entry.lines_for(span.range.clone()) {
                let text = &entry.source.content[l.range.clone()];
                writeln!(w, " {} | {}", l.line_number, text)?;
                let mut underline = String::new();
                let start = l.range.start;
                let end = l.range.end;
                for i in start..end {
                    if i == span.range.start {
                        underline.push('^');
                    } else if i > span.range.start && i < span.range.end {
                        underline.push('~');
                    } else {
                        underline.push(' ');
                    }
                }
                writeln!(w, "   | {}", underline)?;
            }
        }
    }
    Ok(())
}

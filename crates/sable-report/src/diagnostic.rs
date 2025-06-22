use sable_common::RangeUsize;

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

use std::collections::HashMap;
use std::rc::Rc;

use sable_common::{Source, SourceManager, RangeUsize};

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
        let mut start = 0;
        for (idx, ch) in source.content.char_indices() {
            if ch == '\n' {
                lines.push(Line { range: start..idx, line_number: lines.len() + 1 });
                start = idx + 1;
            }
        }
        if start < source.content.len() {
            lines.push(Line { range: start..source.content.len(), line_number: lines.len() + 1 });
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
    manager: &'a SourceManager,
    entries: HashMap<String, CacheEntry>,
}

impl<'a> Cache<'a> {
    pub fn new(manager: &'a SourceManager) -> Self {
        Self { manager, entries: HashMap::new() }
    }

    pub fn get_entry(&mut self, name: &str) -> Option<&CacheEntry> {
        if !self.entries.contains_key(name) {
            if let Some(src) = self.manager.get_content(name) {
                self.entries.insert(name.to_string(), CacheEntry::new(src));
            } else {
                return None;
            }
        }
        self.entries.get(name)
    }
}

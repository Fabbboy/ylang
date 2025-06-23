use std::{
  collections::HashMap,
  ops::Range,
  sync::Arc,
};

use getset::Getters;
use sable_common::source::Source;

use bumpalo::{
  Bump,
  collections::Vec as BumpVec,
};

#[derive(Getters)]
pub struct Line {
  #[getset(get = "pub")]
  range: Range<usize>,
  #[getset(get = "pub")]
  num: usize,
}

impl Line {
  pub fn new(range: Range<usize>, num: usize) -> Self {
    Self { range, num }
  }
}

#[derive(Getters)]
pub struct CacheEntry<'ctx> {
  #[getset(get = "pub")]
  source: Arc<Source<'ctx>>,
  #[getset(get = "pub")]
  lines: BumpVec<'ctx, Line>,
}

impl<'ctx> CacheEntry<'ctx> {
  pub fn new(source: Arc<Source<'ctx>>, bump: &'ctx Bump) -> Self {
    let mut lines = BumpVec::new_in(bump);
    let mut start = 0;
    for (i, c) in source.content().char_indices() {
      if c == '\n' {
        lines.push(Line::new(Range { start, end: i }, lines.len() + 1));
        start = i + 1;
      }
    }

    if start < source.content().len() {
      lines.push(Line::new(
        Range {
          start,
          end: source.content().len(),
        },
        lines.len() + 1,
      ));
    }

    Self { source, lines }
  }

  pub fn get_lines(&self, range: Range<usize>) -> Option<&[Line]> {
    let mut start_idx = None;
    let mut end_idx = None;

    for (i, line) in self.lines.iter().enumerate() {
      if start_idx.is_none() && line.range.end >= range.start {
        start_idx = Some(i);
      }
      if line.range.start <= range.end {
        end_idx = Some(i);
      }
    }

    match (start_idx, end_idx) {
      (Some(s), Some(e)) if s <= e => Some(&self.lines[s..=e]),
      _ => None,
    }
  }
}

#[derive(Getters)]
pub struct Cache<'ctx> {
  #[getset(get = "pub")]
  files: HashMap<&'ctx str, CacheEntry<'ctx>>,
  bump: &'ctx Bump,
}

impl<'ctx> Cache<'ctx> {
  pub fn new(bump: &'ctx Bump) -> Self {
    Self {
      files: HashMap::new(),
      bump,
    }
  }

  pub fn add_file(&mut self, filename: &'ctx str, source: Arc<Source<'ctx>>) {
    if !self.files.contains_key(filename) {
      let entry = CacheEntry::new(source, self.bump);
      self.files.insert(filename, entry);
    }
  }

  pub fn get_file(&self, filename: &'ctx str) -> Option<&CacheEntry<'ctx>> {
    self.files.get(filename)
  }
}

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
    let lines_iter = self
      .lines
      .iter()
      .filter(|line| line.range.start <= range.end && line.range.end >= range.start);
    if lines_iter.clone().next().is_some() {
      Some(self.lines.as_slice())
    } else {
      None
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

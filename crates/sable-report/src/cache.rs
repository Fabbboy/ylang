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
pub struct CacheEntry<'ctx> {
  #[getset(get = "pub")]
  source: Arc<Source<'ctx>>,
  #[getset(get = "pub")]
  lines: BumpVec<'ctx, Range<usize>>,
}

impl<'ctx> CacheEntry<'ctx> {
  pub fn new(source: Arc<Source<'ctx>>, bump: &'ctx Bump) -> Self {
    let mut lines = BumpVec::new_in(bump);
    let mut start = 0;
    for (i, c) in source.content().char_indices() {
      if c == '\n' {
        lines.push(Range { start, end: i });
        start = i + 1;
      }
    }

    if start < source.content().len() {
      lines.push(Range {
        start,
        end: source.content().len(),
      });
    }

    Self { source, lines }
  }

  pub fn get_lines(&self, range: Range<usize>) -> Option<&Range<usize>> {
    self
      .lines
      .iter()
      .find(|&line| line.start <= range.start && line.end >= range.end)
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

use std::ops::Range;

use getset::Getters;

use crate::cache::Cache;

const INDENT_BEFORE: usize = 2;

#[derive(Getters)]
pub struct Span<'ctx> {
  #[getset(get = "pub")]
  range: Range<usize>,
  #[getset(get = "pub")]
  filename: &'ctx str,
}

impl<'ctx> Span<'ctx> {
  pub fn new(range: Range<usize>, filename: &'ctx str) -> Self {
    Self { range, filename }
  }

  pub fn write(&self, cache: &Cache<'ctx>, out: &mut dyn std::io::Write) -> std::io::Result<()> {
    let entry = match cache.get_file(self.filename) {
      Some(entry) => entry,
      None => return Ok(()),
    };

    let lines = match entry.get_lines(self.range.clone()) {
      Some(range) => range,
      None => return Ok(()),
    };

    if let Some(first_line) = lines.first() {
      let column = (self.range.start - first_line.range().start) + 1;
      let line_number = first_line.num();
      writeln!(out, "[{} --> {}:{}]:", self.filename, line_number, column)?;
    } else {
      writeln!(out, "[{}]:", self.filename)?;
    }

    for line in lines {
      let line_number = line.num();
      let raw_src = entry.source().content();
      let line_content = &raw_src[line.range().start..line.range().end];

      writeln!(
        out,
        "{:>width$} | {}",
        line_number,
        line_content,
        width = INDENT_BEFORE
      )?;
    }

    Ok(())
  }
}

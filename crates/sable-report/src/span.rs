use std::ops::Range;

use colored::Colorize;
use getset::Getters;
use pretty::RcDoc;

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

  pub fn to_doc(&self, cache: &Cache<'ctx>) -> RcDoc<'ctx> {
    let entry = match cache.get_file(self.filename) {
      Some(entry) => entry,
      None => return RcDoc::nil(),
    };

    let lines = match entry.get_lines(self.range.clone()) {
      Some(range) => range,
      None => return RcDoc::nil(),
    };

    let mut doc = RcDoc::nil();
    if let Some(first_line) = lines.first() {
      let column = (self.range.start - first_line.range().start) + 1;
      let line_number = first_line.num();
      let header = format!(
        " --> {}:{}:{}",
        self.filename,
        line_number,
        column
      )
      .dimmed()
      .to_string();
      doc = doc.append(RcDoc::text(header)).append(RcDoc::line());
    } else {
      let header = format!(" --> {}:", self.filename).dimmed().to_string();
      doc = doc.append(RcDoc::text(header)).append(RcDoc::line());
    }

    doc = doc.append(RcDoc::text("  |".dimmed().to_string())).append(RcDoc::line());

    for line in lines {
      let line_number = line.num();
      let raw_src = entry.source().content();
      let line_content = &raw_src[line.range().start..line.range().end];

      let main_line = format!("{:>width$} | {}", line_number, line_content, width = INDENT_BEFORE);
      doc = doc.append(RcDoc::text(main_line)).append(RcDoc::line());

      let start = self.range.start.max(line.range().start) - line.range().start;
      let end = self.range.end.min(line.range().end) - line.range().start;
      let underline_len = end.saturating_sub(start).max(1);
      let prefix: String = line_content
        .chars()
        .take(start)
        .map(|c| if c == '\t' { '\t' } else { ' ' })
        .collect();
      let underline = format!("{}{}", prefix, "^".repeat(underline_len));
      let underline = underline.red().bold().to_string();
      let underline_line = format!("{:>width$} | {}", "", underline, width = INDENT_BEFORE);
      doc = doc.append(RcDoc::text(underline_line)).append(RcDoc::line());
    }

    doc
  }

  pub fn write(&self, cache: &Cache<'ctx>, out: &mut dyn std::io::Write) -> std::io::Result<()> {
    self.to_doc(cache).render(100, out)
  }
}

pub mod numeric_error;
pub mod unknown_char;

#[cfg(test)]
mod tests {
  use ariadne::{
    Label,
    Report,
    ReportKind,
    Source,
  };

  #[test]
  fn test_unicode() {
    const SOURCE: &str = r#" preceding 🤨 following"#;
    let report = Report::build(ReportKind::Error, 11..15)
      .with_message("Unexpected character")
      .with_label(Label::new(11..12).with_message("Unexpected character here"))
      .finish();
    report.print(Source::from(SOURCE)).unwrap();
  }
}

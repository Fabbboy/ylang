use std::io;

use bumpalo::Bump;
use sable_ast::location::Location;
use sable_common::manager::Manager;
use sable_parser::{
  lexer::Lexer,
  token::{
    TokenError,
    TokenKind,
  },
};
use sable_report::{
  cache::Cache,
  diagnostic::{
    Diagnostic,
    DiagnosticLevel,
  },
  sink::Sink,
  span::Span,
  writer::DiagnosticWriter,
};

const SOURCE: &str = r#"
   // this is a comment
    let idio = 123; #
"#;

fn report<'f>(err: TokenError, loc: Location<'f>) -> Diagnostic<'f> {
  match err {
    TokenError::UnknownCharacter => Diagnostic::builder()
      .level(DiagnosticLevel::Error)
      .message(Some("Unknown character encountered"))
      .code(Some(Span::new(loc.range().clone(), loc.filename())))
      .build(),
    TokenError::InvalidInteger => Diagnostic::builder()
      .level(DiagnosticLevel::Error)
      .message(Some("Invalid integer literal"))
      .code(Some(Span::new(loc.range().clone(), loc.filename())))
      .build(),
    TokenError::InvalidFloat => Diagnostic::builder()
      .level(DiagnosticLevel::Error)
      .message(Some("Invalid float literal"))
      .code(Some(Span::new(loc.range().clone(), loc.filename())))
      .build(),
  }
}

fn main() {
  let bump = Bump::new();

  let mut manager = Manager::new();
  let mut cache = Cache::new(&bump);
  let source = manager.add_source(SOURCE, "test.sable", &bump);
  cache.add_file("test.sable", source);

  let mut binding = io::stdout();
  let mut writer = DiagnosticWriter::new(&cache, &mut binding);

  let source = manager
    .sources()
    .get("test.sable")
    .expect("Source not found");

  let mut lexer = Lexer::new(source);
  while let Some(token) = lexer.next() {
    println!("{:?}", token);
    match token.kind().clone() {
      TokenKind::Error(err) => {
        let loc = token.location();
        let diagnostic = report(err, loc.clone());
        if let Err(e) = writer.report(diagnostic) {
          eprintln!("Error writing diagnostic: {:?}", e);
        }
      }
      TokenKind::Eof => break,
      _ => {}
    }
  }
}

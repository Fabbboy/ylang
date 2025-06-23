use std::io;

use bumpalo::Bump;
use sable_ast::{
  ast::Ast,
  location::Location,
};
use sable_common::manager::Manager;
use sable_parser::{
  lexer::Lexer,
  parser::{
    ParseStatus,
    Parser,
  },
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
  cache.add_file("test.sable", source.clone());

  let mut binding = io::stdout();
  let mut writer = DiagnosticWriter::new(&cache, &mut binding);

  let mut lexer = Lexer::new(source.as_ref());
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

  let mut ast = Ast::new();
  let mut parser = Parser::new(&writer, lexer, &mut ast);
  match parser.parse() {
    ParseStatus::Success => {
      println!("Parsing completed successfully.");
    }
    ParseStatus::Error => {
      eprintln!("Parsing encountered errors.");
    }
  }
}

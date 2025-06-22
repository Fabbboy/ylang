use std::rc::Rc;
use sable_common::Manager;
use sable_parser::{Lexer, Parser, ParserStatus};
use sable_ast::Ast;
use sable_report::{Cache, StreamWriter, DiagnosticEngine};

fn main() {
    let source = "sasd\nint a = 123; @";
    let mut manager = Manager::new();
    let src = manager.add_content(source, "main.sable");

    let mut cache = Cache::new(&manager);
    cache.add_entry(Rc::clone(&src));

    let lexer = Lexer::new(Rc::clone(&src));
    let mut ast = Ast::new();
    let mut writer = StreamWriter::new(&mut std::io::stdout(), &cache);
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        ParserStatus::Ok => println!("Parsing completed successfully."),
        ParserStatus::OhNo => println!("Parsing failed."),
    }
}

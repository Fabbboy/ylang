use sable_common::SourceManager;
use sable_parser::{Lexer, Parser, ParserStatus};
use sable_ast::Ast;
use sable_report::{StreamWriter, DiagnosticEngine};

fn main() {
    let source = "sasd\nint a = 123; @";
    let mut manager = SourceManager::new();
    let src = manager.add_content(source, "main.sable");

    let lexer = Lexer::new(src);
    let mut _ast = Ast::new();
    let mut writer = StreamWriter::new(&mut std::io::stdout(), &manager);
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        ParserStatus::Ok => println!("Parsing completed successfully."),
        ParserStatus::OhNo => println!("Parsing failed."),
    }
}

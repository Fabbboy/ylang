#include <parsing/Parser/Parser.h>

namespace sable::parsing {
Parser::Parser(const Lexer &lexer, Ast &ast,
               report::DiagnosticEngine &diagnosticEngine)
    : lexer(lexer), ast(ast), diagnosticEngine(diagnosticEngine) {}

Parser::ParserStatus Parser::parse() {
  auto status = ParserStatus::Ok;

  return status;
}
} // namespace sable::parsing
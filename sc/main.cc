#include "common/Manager.h"
#include "parsing/Lexer/Lexer.h"
#include "report/Diagnostic.h"
#include "report/Engine.h"
#include "report/Span.h"
#include <iostream>
#include <memory>
#include <string_view>

using namespace sable::parsing;

std::string_view SOURCE = R"(
 int a = 123;
)";

int main() {
  sable::common::Manager manager;
  auto source = manager.addContent(SOURCE, "main.sable");

  Lexer lexer(source);

  Token token;
  do {
    token = lexer.next();
    std::cout << "Token: " << Token::type_to_string(token.type) << ", Lexeme: '"
              << token.lexeme << "', Location: " << token.location << std::endl;
  } while (token.type != Token::Type::Eof);

  sable::report::StreamWriter<sable::report::FileLocSpan> writer(std::cout);

  sable::report::Diagnostic<sable::report::FileLocSpan> diag(
      sable::report::Severity::Info);
  diag.withMessage("Lexing completed successfully.");
  sable::report::FileLocSpan span(
      source->filename, sable::common::Range<std::size_t>(0, SOURCE.size()));

  diag.withCode(std::make_unique<sable::report::FileLocSpan>(span));
}
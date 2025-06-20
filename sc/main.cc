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
 int a = 123; @
)";

int main() {
  sable::common::Manager manager;
  auto source = manager.addContent(SOURCE, "main.sable");

  Lexer lexer(source);
  sable::report::StreamWriter<sable::report::FileLocSpan> writer(std::cout,
                                                                 manager);

  Token token;
  do {
    token = lexer.next();
    std::cout << "Token: " << Token::type_to_string(token.type) << ", Lexeme: '"
              << token.lexeme << "', Location: " << token.location << std::endl;
    if (token.type == Token::Type::Unknown) {
      sable::report::Diagnostic<sable::report::FileLocSpan> diag(
          sable::report::Severity::Error);
      diag.withMessage(std::string("Unknown token encountered: ") +
                       std::string(token.lexeme));
      sable::report::FileLocSpan span(source->filename, token.location.range);
      auto label =
          sable::report::Label<sable::report::FileLocSpan>(span).withMessage(
              "Invalid token found.");
      diag.withCode(span).withLabel(label);
      writer.report(diag);
    }

  } while (token.type != Token::Type::Eof);
}
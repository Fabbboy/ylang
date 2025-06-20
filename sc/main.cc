#include "common/Manager.h"
#include "parsing/Lexer/Lexer.h"
#include "report/Cache.h"
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

  sable::report::Cache cache(manager);

  Lexer lexer(source);
  sable::report::StreamWriter writer(std::cout, cache);

  Token token;
  do {
    token = lexer.next();
    if (token.type == Token::Type::Unknown) {
      sable::report::Diagnostic diag(sable::report::Severity::Error);
      diag.withMessage(std::string("Unknown token encountered: ") +
                       std::string(token.lexeme));
      sable::report::Span span(source->filename, token.location.range);
      auto label = sable::report::Label(span).withMessage("Invalid token found.");
      diag.withCode(span).withLabel(label);
      writer.report(diag);
    }

  } while (token.type != Token::Type::Eof);
}

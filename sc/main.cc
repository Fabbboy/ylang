#include "common/Manager.h"
#include "common/Range.h"
#include "parsing/Lexer/Lexer.h"
#include "report/Cache.h"
#include "report/Diagnostic.h"
#include "report/Engine.h"
#include "report/Span.h"
#include <format>
#include <iostream>
#include <memory>
#include <string_view>

using namespace sable::parsing;

std::string_view SOURCE = R"(
sasd
int a = 123; @ 
)";

int main() {
  sable::common::Manager manager;
  auto source = manager.addContent(SOURCE, "main.sable");

  sable::report::Cache cache(manager);
  cache.addEntry(source);

  Lexer lexer(source);
  sable::report::StreamWriter writer(std::cout, cache);

  {
    sable::report::Span span(source->filename,
                             sable::common::Range<std::size_t>(0, 4));
    sable::report::Diagnostic info(sable::report::Severity::Info);
    info.withMessage("Demonstrating info diagnostic").withCode(span);
    writer.report(info);
  }

  {
    sable::report::Span span(source->filename,
                             sable::common::Range<std::size_t>(13, 16));
    sable::report::Diagnostic warn(sable::report::Severity::Warning);
    warn.withMessage("This number looks suspicious").withCode(span);
    writer.report(warn);
  }

  Token token;
  do {
    token = lexer.next();
    if (token.type == Token::Type::Unknown ||
        token.type == Token::Type::IntegerError) {

      sable::report::Diagnostic diag(sable::report::Severity::Error);
      diag.withMessage(std::format(
          "Invalid character '{}' found in source code.", token.lexeme));
      sable::report::Span span(source->filename, token.location.range);
      auto label =
          sable::report::Label(span).withMessage("Invalid token found.");
      diag.withCode(span).withLabel(label);
      writer.report(diag);
    }

  } while (token.type != Token::Type::Eof);
}

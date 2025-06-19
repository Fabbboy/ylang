#include "parsing/Lexer/Lexer.h"
#include "parsing/Manager.h"
#include "report/Cache.h"
#include "report/Reporter.h"
#include <iostream>
#include <string_view>
#include <vector>

using namespace ylang::parsing;
using namespace ylang::report;

std::string_view SOURCE = R"(
s
s
s
s
s
 int a = 123 @
)";

int main() {
  Manager manager;
  auto source = manager.addContent(SOURCE, "main.y");
  ReportCache reportCache;
  reportCache.addSource(source);

  Lexer lexer(source);
  ConsoleReporter reporter(reportCache);

  Token token;
  do {
    token = lexer.next();
    if (token.type == Token::Type::Unknown) {
      Location loc = token.location;
      loc.start = 0; // simulate multiline or multi-character span
      BasicDiagnostic diag(
          Severity::Error, std::string("unexpected character"),
          std::vector<Label>{Label{loc, ""}});
      reporter.report(diag);
    }
  } while (token.type != Token::Type::Eof);
}

#include "parsing/Lexer/Lexer.h"
#include "parsing/Manager.h"
#include "report/Cache.h"
#include "report/Reporter.h"
#include <iostream>
#include <string_view>

using namespace ylang::parsing;
using namespace ylang::report;

std::string_view SOURCE = R"(
 int a = 123 @
)";

int main() {
  Manager manager;
  auto source = manager.addContent(SOURCE, "main.y");
  ReportCache reportCache;
  reportCache.addSource(source);

  Lexer lexer(source);
  TextReporter reporter(reportCache);

  Token token;
  do {
    token = lexer.next();
    if (token.type == Token::Type::Unknown) {
      BasicDiagnostic diag(Severity::Error, "unexpected character", token.location);
      reporter.report(diag);
    }
  } while (token.type != Token::Type::Eof);
}
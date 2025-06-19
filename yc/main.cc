#include "parsing/Lexer/Lexer.h"
#include "parsing/Manager.h"
#include "report/Cache.h"
#include "report/Diagnostic.h"
#include "report/Reporter.h"
#include <string_view>
#include <vector>

using namespace ylang::parsing;
using namespace ylang::report;

std::string_view SOURCE = R"(
  $
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
      Label err_lbl;
      err_lbl.loc = token.location;
      err_lbl.message = "unknown token";
      reporter.report(
          BasicDiagnostic(Severity::Error, "Lexical error", {err_lbl}));
    }
  } while (token.type != Token::Type::Eof);

  return 0;
}

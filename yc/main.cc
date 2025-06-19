#include "parsing/Lexer/Lexer.h"
#include "parsing/Manager.h"
#include "report/Cache.h"
#include <iostream>
#include <string_view>

using namespace ylang::parsing;
using namespace ylang::report;

std::string_view SOURCE = R"(
 int a = 123;
)";

int main() {
  Manager manager;
  auto source = manager.addContent(SOURCE, "main.y");
  ReportCache reportCache;
  reportCache.addSource(source);

  Lexer lexer(source);

  Token token;
  do {
    token = lexer.next();
    std::cout << "Token: " << Token::type_to_string(token.type) << ", Lexeme: '"
              << token.lexeme << "', Location: " << token.location << std::endl;

    std::optional<std::reference_wrapper<const SourceCache>> srcCache =
        reportCache.getSource(token.location.file->filename);
    if (srcCache) {
      auto line = srcCache->get().getLine(token.location.start);
      std::string_view lineContent = std::string_view(
          token.location.file->content.c_str() + line->get().start,
          line->get().stop - line->get().start);
      std::cout << "Line: " << line->get().line << ", Content: '" << lineContent
                << "'" << std::endl;
    } else {
      std::cout << "Source cache not found for token location." << std::endl;
    }

  } while (token.type != Token::Type::Eof);
}
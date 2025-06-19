#include "common/Manager.h"
#include "parsing/Lexer/Lexer.h"
#include "report/Diagnostic.h"
#include "report/Engine.h"
#include <iostream>
#include <string_view>

using namespace ylang::parsing;

std::string_view SOURCE = R"(
 int a = 123;
)";

int main() {
  ylang::common::Manager manager;
  auto source = manager.addContent(SOURCE, "main.y");

  Lexer lexer(source);

  Token token;
  do {
    token = lexer.next();
    std::cout << "Token: " << Token::type_to_string(token.type) << ", Lexeme: '"
              << token.lexeme << "', Location: " << token.location << std::endl;
  } while (token.type != Token::Type::Eof);

  ylang::report::StreamWriter writer(std::cout);

  ylang::report::Diagnostic diag(ylang::report::Severity::Info);
  diag.withMessage("Lexing completed successfully.");
  writer.report(diag);
}
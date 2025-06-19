#include "parsing/Lexer/Lexer.h"
#include "parsing/Manager.h"
#include <iostream>
#include <string_view>

using namespace ylang::parsing;

std::string_view SOURCE = R"(
 int a = 123;
)";

int main() {
  Manager manager;
  auto source = manager.addContent(SOURCE, "main.y");
  Lexer lexer(source);

  Token token;
  do {
    token = lexer.next();
    std::cout << "Token: " << Token::type_to_string(token.type) << ", Lexeme: '"
              << token.lexeme << "', Location: " << token.location << std::endl;

  } while (token.type != Token::Type::Eof);
}
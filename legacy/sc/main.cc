#include "common/Manager.h"
#include "parsing/Ast/Ast.h"
#include "parsing/Lexer/Lexer.h"
#include "parsing/Parser/Parser.h"
#include "report/Cache.h"
#include "report/Engine.h"
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
  Ast ast;

  Parser parser(lexer, ast, writer);
  switch (parser.parse()) {
  case Parser::ParserStatus::Ok:
    std::cout << "Parsing completed successfully.\n";
    break;
  case Parser::ParserStatus::OhNo:
    std::cout << "Parsing failed.\n";
    break;
  }
}

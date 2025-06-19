#include "parsing/Lexer/Lexer.h"
#include "parsing/Manager.h"
#include <string_view>

std::string_view SOURCE = R"(

)";

int main() {
  ylang::parsing::Manager manager;
  auto source = manager.addContent(SOURCE, "yc/main.cc");
  ylang::parsing::Lexer lexer(source);
}
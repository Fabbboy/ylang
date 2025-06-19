#pragma once

#include "parsing/Ast/Ast.h"
namespace ylang::parsing {
class Parser {
private:
  Ast &ast;

public:
  Parser(Ast &ast);
};
} // namespace ylang::parsing
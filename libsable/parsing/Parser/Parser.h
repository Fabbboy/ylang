#pragma once

#include "parsing/Ast/Ast.h"
namespace sable::parsing {
class Parser {
private:
  Ast &ast;

public:
  Parser(Ast &ast);
};
} // namespace sable::parsing
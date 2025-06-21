#pragma once

#include "parsing/Ast/Ast.h"
#include "parsing/Lexer/Lexer.h"
#include "report/Engine.h"

namespace sable::parsing {
class Parser {
public:
  enum class ParserStatus {
    Ok,
    OhNo,
  };

private:
  Lexer lexer;
  Ast &ast;
  report::DiagnosticEngine &diagnosticEngine;

public:
  Parser(const Lexer &lexer, Ast &ast,
         report::DiagnosticEngine &diagnosticEngine);

  ParserStatus parse();
};
} // namespace sable::parsing
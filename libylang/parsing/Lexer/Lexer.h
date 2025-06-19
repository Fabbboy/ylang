#pragma once

#include "parsing/Manager.h"
#include <cstddef>
#include <memory>
#include <optional>
#include <parsing/Lexer/Token.h>
#include <string_view>

namespace ylang::parsing {
class Lexer {
private:
  std::string_view source;
  std::shared_ptr<Source> source_ptr;

  std::optional<Token> cache;

  std::size_t start, pos;

private:
  Token lex();
  Location make_location() const;
  std::string_view make_lexeme() const;
  Token make_token(Token::Type type) const;

public:
  Lexer(std::shared_ptr<Source> source_ptr);

  Token next();
};
} // namespace ylang::parsing
#pragma once

#include "common/Manager.h"
#include <cstddef>
#include <functional>
#include <memory>
#include <optional>
#include <parsing/Lexer/Token.h>
#include <string_view>

namespace sable::parsing {
class Lexer {
private:
  std::string_view source;
  std::shared_ptr<common::Source> source_ptr;

  std::optional<Token> cache;

  std::size_t start, pos;

private:
  Token lex();
  Location make_location() const;
  std::string_view make_lexeme() const;
  Token make_token(Token::Type type,
                   std::optional<Token::Data> data = std::nullopt) const;
  std::optional<char> get_char(std::size_t offset = 0) const;
  bool check_char(std::size_t offset,
                  std::function<bool(char)> predicate) const;
  inline void advance() { pos++; }

private:
  void skip_trivial();
  Token lex_identifier();
  Token lex_number();

public:
  Lexer(std::shared_ptr<common::Source> source_ptr);

  Token next();
};
} // namespace sable::parsing
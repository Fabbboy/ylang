#include <optional>
#include <parsing/Lexer/Lexer.h>

namespace ylang::parsing {
Lexer::Lexer(std::shared_ptr<Source> source_ptr)
    : source_ptr(std::move(source_ptr)), cache(std::nullopt), start(0), pos(0) {
  source = this->source_ptr->content;
}

Location Lexer::make_location() const {
  return Location(source_ptr, start, pos);
}

std::string_view Lexer::make_lexeme() const {
  return source.substr(start, pos - start);
}

Token Lexer::make_token(Token::Type type) const {
  return Token(type, make_location(), make_lexeme());
}

Token Lexer::lex() { return make_token(Token::Type::Eof); }

Token Lexer::next() {
  if (!cache.has_value()) {
    cache = lex();
  }

  auto tmp = cache;
  cache = lex();
  return *tmp;
}

} // namespace ylang::parsing
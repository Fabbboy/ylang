#include <cctype>
#include <charconv>
#include <optional>
#include <parsing/Lexer/Lexer.h>

namespace ylang::parsing {
Lexer::Lexer(std::shared_ptr<Source> source_ptr)
    : source_ptr(std::move(source_ptr)), cache(std::nullopt), start(0), pos(0) {
  source = this->source_ptr->content;
}

Location Lexer::make_location() const {
  return Location(source_ptr, Range<std::size_t>(start, pos));
}

std::string_view Lexer::make_lexeme() const {
  return source.substr(start, pos - start);
}

Token Lexer::make_token(Token::Type type,
                        std::optional<Token::Data> data) const {
  return Token(type, make_location(), make_lexeme(), data);
}

std::optional<char> Lexer::get_char(std::size_t offset) const {
  if (pos + offset >= source.size()) {
    return std::nullopt;
  }
  return source[pos + offset];
}

bool Lexer::check_char(std::size_t offset,
                       std::function<bool(char)> predicate) const {
  auto c_opt = get_char(offset);
  if (!c_opt.has_value()) {
    return false;
  }
  return predicate(*c_opt);
}

void Lexer::skip_trivial() {
  while (true) {
    auto c_opt = get_char();
    if (!c_opt.has_value()) {
      return;
    }
    char c = *c_opt;
    switch (c) {
    case ' ':
    case '\t':
    case '\r':
    case '\n':
      advance();
      break;
    default:
      return;
    }
  }
}

Token Lexer::lex_identifier() {
  while (true) {
    auto c_opt = get_char();
    if (!c_opt.has_value()) {
      break;
    }
    char c = *c_opt;
    if (std::isalnum(c) || c == '_') {
      advance();
    } else {
      break;
    }
  }

  return make_token(Token::Type::Identifier);
}

Token Lexer::lex_number() {
  auto lex_num = [this]() -> void {
    while (true) {
      auto c_opt = get_char();
      if (!c_opt.has_value()) {
        break;
      }
      char c = *c_opt;
      if (std::isdigit(c)) {
        advance();
      } else {
        break;
      }
    }
  };

  lex_num();

  if (get_char() == '.' &&
      check_char(1, [](char c) { return std::isdigit(c); })) {
    advance();
    while (true) {
      lex_num();

      auto lexeme = make_lexeme();
      double fval = 0.0f;
      auto [ptr, ec] =
          std::from_chars(lexeme.data(), lexeme.data() + lexeme.size(), fval);
      if (ec != std::errc()) {
        return make_token(Token::Type::FloatError);
      }

      return make_token(Token::Type::Float, Token::Data(fval));
    }
  }

  auto lexeme = make_lexeme();
  int64_t ival = 0;
  auto [ptr, ec] =
      std::from_chars(lexeme.data(), lexeme.data() + lexeme.size(), ival);
  if (ec != std::errc()) {
    return make_token(Token::Type::IntegerError);
  }
  return make_token(Token::Type::Integer, Token::Data(ival));
}

Token Lexer::lex() {
  skip_trivial();

  start = pos;
  auto c_opt = get_char();
  if (!c_opt.has_value()) {
    return make_token(Token::Type::Eof);
  }
  advance();

  switch (*c_opt) {
  case 'a' ... 'z':
  case 'A' ... 'Z':
  case '_':
    return lex_identifier();
  case '0' ... '9':
    return lex_number();
  case ',':
    return make_token(Token::Type::Comma);
  case ';':
    return make_token(Token::Type::Semicolon);
  case '+':
    return make_token(Token::Type::Plus);
  case '-':
    return make_token(Token::Type::Minus);
  case '*':
    return make_token(Token::Type::Star);
  case '/':
    return make_token(Token::Type::Slash);
  case '=':
    return make_token(Token::Type::Assign);
  default:
    return make_token(Token::Type::Unknown);
  }
}

Token Lexer::next() {
  if (!cache.has_value()) {
    cache = lex();
  }

  auto tmp = cache;
  cache = lex();
  return *tmp;
}

} // namespace ylang::parsing
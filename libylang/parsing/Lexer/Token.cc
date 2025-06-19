#include <parsing/Lexer/Token.h>

namespace ylang::parsing {

Token::Token(Type type, Location location, std::string_view lexeme)
    : type(type), location(location), lexeme(lexeme) {}

} // namespace ylang::parsing
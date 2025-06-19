#include <parsing/Lexer/Token.h>

namespace ylang::parsing {

Token::Token(Type type, Location location, std::string_view text)
    : type(type), location(location), text(text) {}

} // namespace ylang::parsing
#include <parsing/Lexer/Token.h>

namespace ylang::parsing {

Token::Token(Type type, Location location, std::string_view lexeme,
             std::optional<Data> data)
    : type(type), location(location), lexeme(lexeme), data(data) {}

} // namespace ylang::parsing
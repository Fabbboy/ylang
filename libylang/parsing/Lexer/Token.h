#include <parsing/Location.h>
#include <string_view>

namespace ylang::parsing {
struct Token {
public:
#define KINDS                                                                  \
  K(Eof)                                                                       \
  K(Unknown)

  enum class Type {
#define K(name) name,
    KINDS
#undef K
  };

  constexpr static std::string_view type_to_string(Type type) {
    switch (type) {
#define K(name)                                                                \
  case Type::name:                                                             \
    return #name;
      KINDS
#undef K
    }
    return "Unknown";
  }

public:
  Type type;
  Location location;
  std::string_view lexeme;

  Token() = default;
  Token(Type type, Location location, std::string_view lexeme);
};
} // namespace ylang::parsing
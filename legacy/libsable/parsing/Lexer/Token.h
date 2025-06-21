#include <cstdint>
#include <optional>
#include <parsing/Location.h>
#include <string_view>

namespace sable::parsing {
struct Token {
public:
#define KINDS                                                                  \
  K(Eof)                                                                       \
  K(Unknown)                                                                   \
  K(IntegerError)                                                              \
  K(FloatError)                                                                \
  K(Identifier)                                                                \
  K(Integer)                                                                   \
  K(Float)                                                                     \
  K(Comma)                                                                     \
  K(Semicolon)                                                                 \
  K(Plus)                                                                      \
  K(Minus)                                                                     \
  K(Star)                                                                      \
  K(Slash)                                                                     \
  K(Assign)

  enum class Type {
#define K(name) name,
    KINDS
#undef K
  };

  union Data {
    int64_t ival;
    double fval;

    Data(int64_t ival) : ival(ival) {}
    Data(double fval) : fval(fval) {}
  };

public:
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
  std::optional<Data> data;

  Token() = default;
  Token(Type type, Location location, std::string_view lexeme,
        std::optional<Data> data);
};
} // namespace sable::parsing
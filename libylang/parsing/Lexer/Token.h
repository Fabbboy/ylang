#include <parsing/Location.h>
#include <string_view>

namespace ylang::parsing {
struct Token {
public:
  enum class Type {

  };

  Type type;
  Location location;
  std::string_view text;

  Token() = default;
  Token(Type type, Location location, std::string_view text);
};
} // namespace ylang::parsing
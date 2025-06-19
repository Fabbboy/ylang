#include <cstddef>
#include <memory>
#include <string>

namespace ylang::parser {
struct Location {
public:
  std::shared_ptr<const std::string> file;
  std::size_t start, stop;

  Location();
};
} // namespace ylang::parser
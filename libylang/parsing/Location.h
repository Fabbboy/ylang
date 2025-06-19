#include "parsing/Manager.h"
#include <cstddef>
#include <memory>

namespace ylang::parsing {
struct Location {
public:
  std::shared_ptr<Source> file;
  std::size_t start, stop;

  Location();
  Location(std::shared_ptr<Source> file, std::size_t start, std::size_t stop);
};
} // namespace ylang::parsing
#include "parsing/Manager.h"
#include <cstddef>
#include <memory>
#include <ostream>

namespace ylang::parsing {
struct Location {
public:
  std::shared_ptr<Source> file;
  std::size_t start, stop;

  Location();
  Location(std::shared_ptr<Source> file, std::size_t start, std::size_t stop);

  friend std::ostream &operator<<(std::ostream &os, const Location &loc);
};
} // namespace ylang::parsing
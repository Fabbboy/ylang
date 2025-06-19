#include "Range.h"
#include "parsing/Manager.h"
#include <cstddef>
#include <memory>
#include <ostream>

namespace ylang::parsing {
struct Location {
public:
  std::shared_ptr<Source> file;
  Range<std::size_t> range;

  Location();
  Location(std::shared_ptr<Source> file, Range<std::size_t> range);

  friend std::ostream &operator<<(std::ostream &os, const Location &loc);
};
} // namespace ylang::parsing
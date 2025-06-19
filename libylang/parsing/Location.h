#include "common/Manager.h"
#include "common/Range.h"
#include <cstddef>
#include <memory>
#include <ostream>

namespace ylang::parsing {
struct Location {
public:
  std::shared_ptr<common::Source> file;
  common::Range<std::size_t> range;

  Location();
  Location(std::shared_ptr<common::Source> file, common::Range<std::size_t> range);

  friend std::ostream &operator<<(std::ostream &os, const Location &loc);
};
} // namespace ylang::parsing
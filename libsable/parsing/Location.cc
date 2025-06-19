#include <parsing/Location.h>

namespace sable::parsing {
Location::Location() : file(nullptr), range(0, 0) {}
Location::Location(std::shared_ptr<sable::common::Source> file, sable::common::Range<std::size_t> range)
    : file(std::move(file)), range(range) {}

std::ostream &operator<<(std::ostream &os, const Location &loc) {
  if (loc.file) {
    os << loc.file->filename << ":";
  }
  os << loc.range.getStart() << "-" << loc.range.getStop();
  return os;
}
} // namespace sable::parsing
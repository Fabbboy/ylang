#include <parsing/Location.h>

namespace ylang::parsing {
Location::Location() : file(nullptr), range(0, 0) {}
Location::Location(std::shared_ptr<common::Source> file, common::Range<std::size_t> range)
    : file(std::move(file)), range(range) {}

std::ostream &operator<<(std::ostream &os, const Location &loc) {
  if (loc.file) {
    os << loc.file->filename << ":";
  }
  os << loc.range.getStart() << "-" << loc.range.getStop();
  return os;
}
} // namespace ylang::parsing
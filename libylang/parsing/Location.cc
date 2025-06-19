#include <parsing/Location.h>

namespace ylang::parsing {
Location::Location() : file(nullptr), start(0), stop(0) {}
Location::Location(std::shared_ptr<Source> file, std::size_t start,
                   std::size_t stop)
    : file(std::move(file)), start(start), stop(stop) {}

std::ostream &operator<<(std::ostream &os, const Location &loc) {
  if (loc.file) {
    os << loc.file->filename << ":";
  }
  os << loc.start << "-" << loc.stop;
  return os;
}
} // namespace ylang::parsing
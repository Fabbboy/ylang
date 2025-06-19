#include <parsing/Location.h>

namespace ylang::parsing {
Location::Location() : file(nullptr), start(0), stop(0) {}
Location::Location(std::shared_ptr<Source> file, std::size_t start,
                   std::size_t stop)
    : file(std::move(file)), start(start), stop(stop) {}
} // namespace ylang::parsing
#include <parsing/Location.h>

namespace ylang::parsing {
Location::Location() : file_id(0), start(0), stop(0) {}
Location::Location(ContentId file_id, std::size_t start, std::size_t stop)
    : file_id(file_id), start(start), stop(stop) {}
} // namespace ylang::parsing
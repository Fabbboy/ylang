#include "parser/Manager.h"
#include <cstddef>

namespace ylang::parser {
struct Location {
public:
  ContentId file_id;
  std::size_t start, stop;

  Location();
  Location(ContentId file_id, std::size_t start, std::size_t stop);
};
} // namespace ylang::parser
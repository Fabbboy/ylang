#include <report/Cache.h>

namespace sable::report {
Line::Line(common::Range<std::size_t> range) : range(range) {}
CacheEntry::CacheEntry(std::shared_ptr<common::Source> source)
    : source(std::move(source)) {
  if (!this->source) {
    return;
  }

  std::size_t start = 0;
  std::size_t end = 0;
  std::string_view content = this->source->content;
  std::size_t length = content.length();
  while (end < length) {
    if (content[end] == '\n') {
      lines.emplace_back(common::Range<std::size_t>(start, end));
      start = end + 1;
    }
    ++end;
  }
}

} // namespace sable::report
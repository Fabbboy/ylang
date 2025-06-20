#include "report/Cache.h"
#include <span>

namespace sable::report {

Line::Line(common::Range<std::size_t> range, std::size_t lineNumber)
    : range(std::move(range)), lineNumber(lineNumber) {}

std::size_t Line::start() const { return range.getStart(); }
std::size_t Line::end() const { return range.getStop(); }
std::size_t Line::length() const { return end() - start(); }
bool Line::isWithin(common::Range<std::size_t> other) const {
  return other.contains(start()) && other.contains(end() - 1);
}

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
      lines.emplace_back(common::Range<std::size_t>(start, end),
                         lines.size() + 1);
      start = end + 1;
    }
    ++end;
  }
}

std::span<const Line>
CacheEntry::getLines(common::Range<std::size_t> range) const {
  std::span<const Line> result;
  if (!source) {
    return result;
  }

  for (const auto &line : lines) {
    if (line.isWithin(range)) {
      if (result.empty()) {
        result = std::span<const Line>(&line, 1);
      } else {
        result = std::span<const Line>(result.data(), result.size() + 1);
      }
    }
  }

  return result;
}

Cache::Cache(const common::Manager &manager) : manager(manager) {}

void Cache::addEntry(std::shared_ptr<common::Source> source) {
  if (!source) {
    return;
  }
  auto it = entries.find(source->filename);
  if (it == entries.end()) {
    entries.emplace(source->filename, CacheEntry(std::move(source)));
  }
}

std::optional<const CacheEntry *>
Cache::getEntry(std::string_view sourceName) const {
  auto it = entries.find(sourceName);
  if (it != entries.end()) {
    return &it->second;
  }
  return std::nullopt;
}

const common::Manager &Cache::getManager() const { return manager; }

} // namespace sable::report

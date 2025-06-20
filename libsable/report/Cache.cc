#include "report/Cache.h"
#include <span>

namespace sable::report {

Line::Line(common::Range<std::size_t> range, std::size_t lineNumber)
    : range(std::move(range)), lineNumber(lineNumber) {}

std::size_t Line::start() const { return range.getStart(); }
std::size_t Line::end() const { return range.getStop(); }
std::size_t Line::length() const { return end() - start(); }
bool Line::isWithin(common::Range<std::size_t> other) const {
  return start() <= other.getStop() && end() > other.getStart();
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

  // Handle the last line if it doesn't end with a newline
  if (start < length) {
    lines.emplace_back(common::Range<std::size_t>(start, length),
                       lines.size() + 1);
  }
}

std::span<const Line>
CacheEntry::getLines(common::Range<std::size_t> range) const {
  if (!source || lines.empty()) {
    return {};
  }

  std::size_t firstIdx = lines.size();
  std::size_t lastIdx = 0;

  for (std::size_t i = 0; i < lines.size(); ++i) {
    const auto &line = lines[i];
    if (line.isWithin(range)) {
      if (firstIdx == lines.size()) {
        firstIdx = i;
      }
      lastIdx = i;
    }
  }

  if (firstIdx == lines.size()) {
    return {};
  }

  std::span<const Line> slice =
      std::span<const Line>(&lines[firstIdx], lastIdx - firstIdx + 1);
  return slice;
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

#include <cstddef>
#include <report/Cache.h>

namespace ylang::report {
Line::Line(std::size_t start, std::size_t stop, std::size_t line)
    : start(start), stop(stop), line(line) {}

bool Line::isWithin(std::size_t offset) const {
  return offset >= start && offset < stop;
}

SourceCache::SourceCache(std::shared_ptr<parsing::Source> source)
    : source(source) {
  construct();
}

void SourceCache::construct() {
  auto src = source->content;

  auto begin = src.begin();
  auto end = src.end();
  std::size_t start = 0, stop = 0, line = 1;
  for (auto it = begin; it != end; ++it) {
    if (*it == '\n') {
      stop = std::distance(begin, it);
      lines.emplace_back(start, stop, line);
      start = stop + 1;
      line++;
    }
  }

  if (start < std::distance(begin, end)) {
    stop = std::distance(begin, end);
    lines.emplace_back(start, stop, line);
  }
}

std::optional<std::reference_wrapper<const Line>>
SourceCache::getLine(std::size_t offset) const {
  for (const auto &line : lines) {
    if (line.isWithin(offset)) {
      return line;
    }
  }
  return std::nullopt;
}

void ReportCache::addSource(std::shared_ptr<parsing::Source> source) {
  auto it = cache.find(source->filename);
  if (it == cache.end()) {
    cache.emplace(source->filename, SourceCache(source));
  }
}

std::optional<std::reference_wrapper<const SourceCache>>
ReportCache::getSource(std::string_view filename) const {
  auto it = cache.find(filename);
  if (it != cache.end()) {
    return it->second;
  }
  return std::nullopt;
}
} // namespace ylang::report

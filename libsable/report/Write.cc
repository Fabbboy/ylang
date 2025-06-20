#include "report/Write.h"

namespace sable::report {

void writeLine(std::ostream &os, const Line &line,
               std::shared_ptr<common::Source> source) {
  os << line.lineNumber << " | ";
  os << source->content.substr(line.start(), line.length());
  os << "\n";
}

void SpanWrite::write(std::ostream &os, const Span &span, const Cache &cache) {
  std::optional<const CacheEntry *> entry = cache.getEntry(span.source());

  if (!entry.has_value()) {
    return;
  }

  std::span<const Line> lines =
      entry.value()->getLines(span.getRange());
  if (lines.empty())
    return;

  const Line &line = lines.front();
  std::size_t line_start = line.start();
  std::size_t column = span.start() - line_start + 1;

  os << "[" << span.source() << ":" << line.lineNumber << ":" << column
     << "] ";

  for (const auto &l : lines) {
    writeLine(os, l, entry.value()->source);
  }
}

} // namespace sable::report

#include "report/Write.h"
#include "report/Diagnostic.h"
#include "report/Span.h"
#include <cstddef>

namespace sable::report {

void writeLine(std::ostream &os, const Span &span, const Line &line,
               std::shared_ptr<common::Source> source,
               std::string_view message) {
  os << " " << line.lineNumber << " | ";
  os << source->content.substr(line.start(), line.length());
  os << "\n   | ";
  std::size_t line_start = line.start();
  std::size_t line_end = line.end();
  std::size_t span_start = span.start();
  std::size_t span_end = span.end();
  std::size_t span_length = span.length();

  for (std::size_t i = line_start; i < line_end; ++i) {
    if (i >= span_start && i < span_end) {
      os << ANSI_YELLOW << "~" << ANSI_RESET;
    } else {
      os << " ";
    }
  }

  os << " " << message << "\n";
}

void writeSpan(std::ostream &os, const Span &span, const Cache &cache,
               std::string_view message) {
  std::optional<const CacheEntry *> entry = cache.getEntry(span.source());

  if (!entry.has_value()) {
    return;
  }

  std::span<const Line> lines = entry.value()->getLines(span.getRange());
  if (lines.empty())
    return;

  const Line &line = lines.front();
  std::size_t line_start = line.start();
  std::size_t column = span.start() - line_start + 1;

  os << "[" << span.source() << ":" << line.lineNumber << ":" << column
     << "]\n";

  for (const auto &l : lines) {
    writeLine(os, span, l, entry.value()->source, message);
  }
}

} // namespace sable::report

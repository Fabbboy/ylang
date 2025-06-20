#include "report/Write.h"
#include "report/Diagnostic.h"
#include "report/Span.h"
#include <cstddef>
#include <string>

namespace sable::report {

namespace {

std::size_t digits(std::size_t num) {
  std::size_t d = 1;
  while (num >= 10) {
    num /= 10;
    ++d;
  }
  return d;
}

std::string underline(const Span &span, const Line &line) {
  std::string result;
  std::size_t line_start = line.start();
  std::size_t line_end = line.end();
  std::size_t span_start = span.start();
  std::size_t span_end = span.end();

  for (std::size_t i = line_start; i < line_end; ++i) {
    if (i == span_start) {
      result += std::string(ANSI_YELLOW) + "^" + ANSI_RESET;
    } else if (i > span_start && i < span_end) {
      result += std::string(ANSI_YELLOW) + "~" + ANSI_RESET;
    } else {
      result += ' ';
    }
  }
  return result;
}

} // namespace

void writeLine(std::ostream &os, const Span &span, const Line &line,
               std::size_t width,
               std::shared_ptr<common::Source> source) {
  os << ' ' << line.lineNumber << " | "
     << source->content.substr(line.start(), line.length()) << '\n';
  os << std::string(width + 1, ' ') << " | " << underline(span, line) << '\n';
}

void writeSpan(std::ostream &os, const Span &span, const Cache &cache) {
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

  std::size_t width = digits(lines.back().lineNumber);
  for (const auto &l : lines) {
    writeLine(os, span, l, width, entry.value()->source);
  }
}

} // namespace sable::report

#pragma once

#include "common/Manager.h"
#include "report/Cache.h"
#include "report/Span.h"
#include <cstddef>
#include <memory>
#include <ostream>

namespace sable::report {
static inline void writeLine(std::ostream &os, const Line &line,
                             std::shared_ptr<common::Source> source) {
  os << line.lineNumber << " | ";
  os << source->content.substr(line.start(), line.length());
  os << "\n";
}

template <typename S> struct SpanWrite {
  static_assert(is_derived_from_span_v<S>, "S must be derived from Span");
  static void write(std::ostream &os, const S &span, const Cache<S> &cache) {
    std::optional<const CacheEntry *> entry = cache.getEntry(span);

    if (!entry.has_value()) {
      return;
    }

    std::span<const Line> lines =
        entry.value()->getLines(span.getRange());
    if (lines.empty())
      return;

    const Line &line = lines.front();
    std::size_t line_start = line.start();
    std::size_t column = span.getStart() - line_start + 1;

    os << "[" << span.source() << ":" << line.lineNumber << ":" << column
       << "] ";

    for (const auto &l : lines) {
      write(os, l);
    }
  }
};
} // namespace sable::report
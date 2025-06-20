#pragma once

#include "common/Manager.h"
#include "report/Cache.h"
#include "report/Span.h"
#include <memory>
#include <ostream>

namespace sable::report {

void writeLine(std::ostream &os, const Line &line,
               std::shared_ptr<common::Source> source);

struct SpanWrite {
  static void write(std::ostream &os, const Span &span, const Cache &cache);
};

} // namespace sable::report

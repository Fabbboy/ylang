#pragma once

#include "common/Manager.h"
#include "report/Cache.h"
#include "report/Span.h"
#include <cstddef>
#include <memory>
#include <ostream>

namespace sable::report {

void writeLine(std::ostream &os, const Span &span, const Line &line,
               std::size_t width,
               std::shared_ptr<common::Source> source);

void writeSpan(std::ostream &os, const Span &span, const Cache &cache);

} // namespace sable::report

#pragma once

#include "common/Manager.h"
#include "report/Cache.h"
#include "report/Span.h"
#include <memory>
#include <ostream>
#include <string_view>

namespace sable::report {

void writeLine(std::ostream &os, const Line &line,
               std::shared_ptr<common::Source> source,
               std::string_view message);

void writeSpan(std::ostream &os, const Span &span, const Cache &cache,
               std::string_view message);

} // namespace sable::report

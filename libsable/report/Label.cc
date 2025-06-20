#include "report/Label.h"
#include "report/Cache.h"
#include "report/Write.h"

namespace sable::report {

Label::Label(Span span) : span(std::move(span)) {}

Label &Label::withMessage(const std::string &msg) {
  message = msg;
  return *this;
}

std::ostream &Label::print(std::ostream &os, const Cache &cache) const {
  writeSpan(os, span, cache, message.value_or(""));
  return os;
}

} // namespace sable::report

#include "report/Label.h"
#include "report/Cache.h"
#include "report/Diagnostic.h"
#include "report/Write.h"

namespace sable::report {

Label::Label(Span span) : span(std::move(span)) {}

Label &Label::withMessage(const std::string &msg) {
  message = msg;
  return *this;
}

std::ostream &Label::print(std::ostream &os, const Cache &cache) const {
  os << ANSI_BLUE "[Note]: " ANSI_RESET;
  if (message) {
    os << *message << "\n";
  }
  writeSpan(os, span, cache);
  return os;
}

} // namespace sable::report

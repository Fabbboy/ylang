#include "report/Label.h"

namespace sable::report {

Label::Label(Span span) : span(std::move(span)) {}

Label &Label::withMessage(const std::string &msg) {
  message = msg;
  return *this;
}

std::ostream &Label::print(std::ostream &os, const common::Manager &manager) const {
  os << span.source() << ":" << span.start() << ":" << span.end();
  if (message) {
    os << " --> " << *message;
  }
  if (span.length() > 0) {
    auto file = manager.getContent(span.source());
    if (file) {
      os << " | " << file.value()->content.substr(span.start(), span.length());
    } else {
      os << " | <unknown content>";
    }
  }
  os << "\n";
  os << "  " << std::string(span.start(), ' ') << "^";
  if (span.length() > 1) {
    os << std::string(span.length() - 1, '~');
  }
  os << "\n";

  return os;
}

} // namespace sable::report

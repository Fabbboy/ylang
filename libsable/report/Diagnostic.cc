#include "report/Diagnostic.h"

namespace sable::report {

std::ostream &operator<<(std::ostream &os, Severity severity) {
  switch (severity) {
#define X(name, color) case Severity::name: os << color << #name << ANSI_RESET; break;
    SEVERITY_LEVELS
#undef X
  default:
    os << "Unknown";
    break;
  }
  return os;
}

Diagnostic::Diagnostic(Severity severity) : severity(severity) {}

Diagnostic &Diagnostic::withMessage(const std::string &msg) {
  message = msg;
  return *this;
}

Diagnostic &Diagnostic::withCode(const Span &span) {
  code = span;
  return *this;
}

Diagnostic &Diagnostic::withLabel(const Label &label) {
  labels.push_back(label);
  return *this;
}

std::ostream &Diagnostic::print(std::ostream &os, const Cache &cache) const {
  os << severity << ": ";
  if (message) {
    os << *message;
  }
  os << "\n";

  if (code) {
    SpanWrite::write(os, *code, cache);
  }

  return os;
}

} // namespace sable::report

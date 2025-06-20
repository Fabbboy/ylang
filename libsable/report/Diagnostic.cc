#include "report/Diagnostic.h"
#include "report/Write.h"
#include <string_view>

namespace sable::report {

std::ostream &operator<<(std::ostream &os, Severity severity) {
  switch (severity) {
#define X(name, color)                                                         \
  case Severity::name:                                                         \
    os << color << #name << ANSI_RESET;                                        \
    break;
    SEVERITY_LEVELS
#undef X
  default:
    os << "Unknown";
    break;
  }
  return os;
}

Diagnostic::Diagnostic(Severity severity) : severity(severity) {}

Diagnostic &Diagnostic::withMessage(std::string_view msg) {
  message = msg;
  return *this;
}

Diagnostic &Diagnostic::withNote(std::string_view note) {
  this->note = note;
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
    writeSpan(os, *code, cache);
  }

  for (const auto &label : labels) {
    label.print(os, cache);
  }

  return os;
}

} // namespace sable::report

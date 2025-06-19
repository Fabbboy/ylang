#include <report/Diagnostic.h>
#include <sstream>

namespace ylang::report {
std::ostream &operator<<(std::ostream &os, Severity severity) {
  switch (severity) {
#define X(name)                                                                \
  case Severity::name:                                                         \
    os << #name;                                                               \
    break;
    SEVERITY_LEVELS
#undef X
  default:
    os << "Unknown";
    break;
  }
  return os;
}

Diagnostic::Diagnostic(Severity severity)
    : severity(severity), message(std::nullopt) {}

std::string Diagnostic::write() const {
  std::stringstream ss;
  ss << "[" << severity << "]: ";
  if (message.has_value()) {
    ss << message.value();
  }

  return ss.str();
}

std::ostream &operator<<(std::ostream &os, const Diagnostic &diag) {
  os << diag.write();
  return os;
}
} // namespace ylang::report
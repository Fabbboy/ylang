#pragma once

#include "report/Span.h"
#include <memory>
#include <optional>
#include <ostream>
#include <string>
namespace sable::report {
#define SEVERITY_LEVELS                                                        \
  X(Info)                                                                      \
  X(Warning)                                                                   \
  X(Error)

enum class Severity {
#define X(name) name,
  SEVERITY_LEVELS
#undef X
};

inline std::ostream &operator<<(std::ostream &os, Severity severity) {
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

template <typename S> class Diagnostic {
private:
  Severity severity;
  std::optional<std::string> message;
  std::optional<std::unique_ptr<Span<S>>> code;

public:
  Diagnostic(Severity severity) : severity(severity) {}

  inline void withMessage(const std::string &msg) { message = msg; }
  inline void withCode(std::unique_ptr<Span<S>> &&span) {
    code = std::move(span);
  }
};

} // namespace sable::report
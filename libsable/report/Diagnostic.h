#pragma once

#include "report/Cache.h"
#include "report/Label.h"
#include "report/Span.h"
#include "report/Write.h"
#include <optional>
#include <ostream>
#include <string>
#include <vector>

#define ANSI_COLOR(color) "\033[" color "m"
#define ANSI_RESET ANSI_COLOR("0")
#define ANSI_RED ANSI_COLOR("31")
#define ANSI_YELLOW ANSI_COLOR("33")
#define ANSI_BLUE ANSI_COLOR("34")

namespace sable::report {

#define SEVERITY_LEVELS                                                        \
  X(Info, ANSI_BLUE)                                                           \
  X(Warning, ANSI_YELLOW)                                                      \
  X(Error, ANSI_RED)

enum class Severity {
#define X(name, color) name,
  SEVERITY_LEVELS
#undef X
};

inline std::ostream &operator<<(std::ostream &os, Severity severity) {
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

template <typename S> class Diagnostic {
  static_assert(is_derived_from_span_v<S>,
                "S must be derived from Span<T> for some type T");

private:
  Severity severity;
  std::optional<std::string> message;
  std::optional<S> code;
  std::vector<Label<S>> labels;

public:
  Diagnostic(Severity severity) : severity(severity) {}

  inline Diagnostic<S> &withMessage(const std::string &msg) {
    message = msg;
    return *this;
  }
  inline Diagnostic<S> &withCode(const S &span) {
    code = span;
    return *this;
  }
  inline Diagnostic<S> &withLabel(const Label<S> &label) {
    labels.push_back(label);
    return *this;
  }

  std::ostream &print(std::ostream &os, const Cache<S> &cache) const {
    os << severity << ": ";
    if (message) {
      os << *message;
    }
    os << "\n";

    if (code) {
      SpanWrite<S>::write(os, *code, cache);
    }

    return os;
  }
};

} // namespace sable::report
#pragma once

#include "common/Manager.h"
#include "report/Label.h"
#include "report/Span.h"
#include <optional>
#include <ostream>
#include <string>
#include <vector>

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

  std::ostream &print(std::ostream &os, const common::Manager &manager) const {
    os << "[" << severity << "] ";
    if (message) {
      os << *message;
    }
    os << "\n";

    if (code) {
      auto file = manager.getContent(code->source());
      if (file) {
        os << file->filename << ":" << code->start() << ":" << code->end()
           << " | " << file->content.substr(code->start(), code->length())
           << "\n";
        os << "  " << std::string(code->start(), ' ') << "^";
        if (code->length() > 1) {
          os << std::string(code->length() - 1, '~');
        }
        os << "\n";
      }
    }
    os << "\n";

    for (const auto &label : labels) {
      label.print(os, manager);
    }

    return os;
  }
};

} // namespace sable::report
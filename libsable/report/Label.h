#pragma once

#include "common/Manager.h"
#include "report/Span.h"
#include <optional>
#include <ostream>
#include <string>

namespace sable::report {
template <typename S> class Label {
  static_assert(is_derived_from_span_v<S>,
                "S must be derived from Span<T> for some type T");

private:
  S span;
  std::optional<std::string> message;

public:
  Label(S span) : span(std::move(span)) {}

  inline Label<S> &withMessage(const std::string &msg) {
    message = msg;
    return *this;
  }

  std::ostream &print(std::ostream &os, const common::Manager &manager) const {
    os << span.source() << ":" << span.start() << ":" << span.end();
    if (message) {
      os << " --> " << *message;
    }
    if (span.length() > 0) {
      auto file = manager.getContent(span.source());
      if (file) {
        os << " | " << file->content.substr(span.start(), span.length());
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
};
} // namespace sable::report
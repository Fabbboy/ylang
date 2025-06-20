#pragma once

#include "report/Span.h"
#include <optional>
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
};
} // namespace sable::report
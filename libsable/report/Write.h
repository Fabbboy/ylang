#pragma once

#include "common/Manager.h"
#include "report/Span.h"
#include <ostream>
#include <string_view>
namespace sable::report {
template <typename S> struct SpanWrite {
  static_assert(is_derived_from_span_v<S>, "S must be derived from Span");
  static void write(std::ostream &os, const S &span,
                    const common::Manager &manager) {
    std::optional<std::shared_ptr<common::Source>> source =
        manager.getContent(span.source());

    if (!source)
      return;

    std::string_view content = source->get()->content;
    std::string_view content_slice =
        content.substr(span.start(), span.length());

    os << "[" << source->get()->filename << ":" << span.start() << "-"
       << span.end() << "]\n";
    os << " | " << content_slice << "\n";
    os << " | ";
    for (std::size_t i = 0; i < span.start(); ++i) {
      os << " ";
    }

    for (std::size_t i = 0; i < span.length(); ++i) {
      os << "^";
    }
    os << "\n";
  }
};
} // namespace sable::report
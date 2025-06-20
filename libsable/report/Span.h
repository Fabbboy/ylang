#pragma once

#include "common/Range.h"
#include <cstddef>
#include <string_view>
#include <functional>

namespace sable::report {

class Span {
private:
  std::string_view source_;
  common::Range<std::size_t> range_;

public:
  Span(std::string_view source, common::Range<std::size_t> range);

  const std::string_view &source() const;
  std::size_t start() const;
  std::size_t end() const;
  std::size_t length() const;
  common::Range<std::size_t> getRange() const;

  bool operator==(const Span &other) const;
};

} // namespace sable::report

namespace std {
template <> struct hash<sable::report::Span> {
  size_t operator()(const sable::report::Span &span) const;
};
} // namespace std

#include "report/Span.h"

namespace sable::report {

Span::Span(std::string_view source, common::Range<std::size_t> range)
    : source_(source), range_(range) {}

const std::string_view &Span::source() const { return source_; }
std::size_t Span::start() const { return range_.getStart(); }
std::size_t Span::end() const { return range_.getStop(); }
std::size_t Span::length() const { return range_.getLength(); }
common::Range<std::size_t> Span::getRange() const { return range_; }

bool Span::operator==(const Span &other) const {
  return source_ == other.source_ &&
         range_.getStart() == other.range_.getStart() &&
         range_.getStop() == other.range_.getStop();
}

} // namespace sable::report

size_t std::hash<sable::report::Span>::operator()(const sable::report::Span &span) const {
  size_t h1 = std::hash<std::string_view>()(span.source());
  size_t h2 = std::hash<std::size_t>()(span.start());
  size_t h3 = std::hash<std::size_t>()(span.end());
  return h1 ^ (h2 << 1) ^ (h3 << 2);
}

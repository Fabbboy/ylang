#pragma once

#include "common/Range.h"
#include <cstddef>
#include <functional>
#include <string_view>
#include <type_traits>
#include <utility>

namespace sable::report {

template <typename S> class Span {
  static_assert(std::is_copy_constructible_v<S>,
                "T must be copy constructible");
  static_assert(std::is_copy_assignable_v<S>, "T must be copy assignable");
  static_assert(std::is_default_constructible_v<std::hash<S>>,
                "std::hash<T> must be defined");
  static_assert(std::is_invocable_r_v<std::size_t, std::hash<S>, S>,
                "std::hash<T> must be callable");
  static_assert(std::is_convertible_v<
                    decltype(std::declval<S>() == std::declval<S>()), bool>,
                "T must support operator==");

public:
  virtual ~Span() = default;

  virtual const S &source() const = 0;
  virtual std::size_t start() const = 0;
  virtual std::size_t end() const = 0;
  virtual std::size_t length() const { return end() - start(); }
};

template <typename T> struct is_derived_from_span {
private:
  template <typename U> static auto test(const Span<U> *) -> std::true_type;
  static auto test(...) -> std::false_type;

public:
  static constexpr bool value = decltype(test(std::declval<T *>()))::value;
};

template <typename T>
inline constexpr bool is_derived_from_span_v = is_derived_from_span<T>::value;

template <typename T> struct span_template_argument {
private:
  template <typename S> static S test(const sable::report::Span<S> *);

  static void test(...);

public:
  using type = decltype(test(std::declval<T *>()));
};

template <typename T>
using span_template_argument_t = typename span_template_argument<T>::type;

class FileLocSpan : public Span<std::string_view> {
private:
  std::string_view source_;
  common::Range<std::size_t> range;

public:
  FileLocSpan(std::string_view source, common::Range<std::size_t> range)
      : source_(std::move(source)), range(range) {}

  const std::string_view &source() const override { return source_; }
  common::Range<std::size_t> getRange() const { return range; }

  bool operator==(const FileLocSpan &other) const {
    return source_ == other.source_ &&
           range.getStart() == other.range.getStart() &&
           range.getStop() == other.range.getStop();
  }
};

} // namespace sable::report

namespace std {
template <> struct hash<sable::report::FileLocSpan> {
  std::size_t operator()(const sable::report::FileLocSpan &span) const {
    std::size_t h1 = std::hash<std::string_view>()(span.source());
    std::size_t h2 = std::hash<std::size_t>()(span.start());
    std::size_t h3 = std::hash<std::size_t>()(span.end());
    return h1 ^ (h2 << 1) ^ (h3 << 2); // Combine the hashes
  }
};
} // namespace std
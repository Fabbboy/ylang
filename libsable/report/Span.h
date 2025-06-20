#pragma once

#include <cstddef>
#include <functional>
#include <type_traits>

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

  virtual S source() const = 0;
  virtual std::size_t start() const = 0;
  virtual std::size_t end() const = 0;
  virtual std::size_t length() const { return end() - start(); }
};
} // namespace sable::report
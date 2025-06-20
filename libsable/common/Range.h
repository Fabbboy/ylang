#pragma once

#include <type_traits>
namespace sable::common {
template <typename T> class Range {
  static_assert(std::is_arithmetic<T>::value,
                "Range can only be used with arithmetic types.");

private:
  T start;
  T stop;

public:
  Range() : start(), stop() {}
  Range(T start, T stop) : start(start), stop(stop) {}

  Range(const Range &other) : start(other.start), stop(other.stop) {}
  Range &operator=(const Range &other) {
    if (this != &other) {
      start = other.start;
      stop = other.stop;
    }
    return *this;
  }

  Range(Range &&other) noexcept : start(other.start), stop(other.stop) {
    other.start = T();
    other.stop = T();
  }

  Range &operator=(Range &&other) noexcept {
    if (this != &other) {
      start = other.start;
      stop = other.stop;
      other.start = T();
      other.stop = T();
    }
    return *this;
  }

  T getStart() const { return start; }
  T getStop() const { return stop; }
  T getLength() const { return stop - start; }

  bool contains(T value) const { return value >= start && value < stop; }
};
} // namespace sable::common
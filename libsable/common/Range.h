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
  Range(T start, T stop) : start(start), stop(stop) {}

  T getStart() const { return start; }
  T getStop() const { return stop; }
  T getLength() const { return stop - start; }

  bool contains(T value) const { return value >= start && value < stop; }
};
} // namespace sable
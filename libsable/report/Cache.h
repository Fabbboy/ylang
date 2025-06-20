#pragma once

#include "common/Manager.h"
#include "common/Range.h"
#include <cstddef>
#include <memory>
#include <unordered_map>

namespace sable::report {
struct Line {
public:
  common::Range<std::size_t> range;

  Line(common::Range<std::size_t> range);
};

struct CacheEntry {
public:
  std::shared_ptr<common::Source> source;
};

template <typename S> class Cache {
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

private:
  const common::Manager &manager;
  std::unordered_map<S, CacheEntry> entries;
};
} // namespace sable::report
#pragma once

#include "common/Manager.h"
#include "common/Range.h"
#include "report/Span.h"
#include <cstddef>
#include <memory>
#include <optional>
#include <span>
#include <unordered_map>
#include <string_view>
#include <vector>

namespace sable::report {

struct Line {
  common::Range<std::size_t> range;
  std::size_t lineNumber;

  Line(common::Range<std::size_t> range, std::size_t lineNumber);
  std::size_t start() const;
  std::size_t end() const;
  std::size_t length() const;
  bool isWithin(common::Range<std::size_t> other) const;
};

struct CacheEntry {
  std::shared_ptr<common::Source> source;
  std::vector<Line> lines;

  CacheEntry(std::shared_ptr<common::Source> source);

  std::span<const Line> getLines(common::Range<std::size_t> range) const;
};

class Cache {
private:
  const common::Manager &manager;
  std::unordered_map<std::string_view, CacheEntry> entries;

public:
  explicit Cache(const common::Manager &manager);

  void addEntry(std::shared_ptr<common::Source> source);
  std::optional<const CacheEntry *> getEntry(const Span &span) const;

  const common::Manager &getManager() const;
};

} // namespace sable::report

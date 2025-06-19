#pragma once

#include "parsing/Manager.h"
#include <cstddef>
#include <functional>
#include <memory>
#include <optional>
#include <unordered_map>
#include <vector>

namespace ylang::report {
struct Line {
public:
  std::size_t start, stop;
  std::size_t line;

  Line(std::size_t start, std::size_t stop, std::size_t line);
  bool isWithin(std::size_t offset) const;
};

class SourceCache {
private:
  std::vector<Line> lines;
  std::shared_ptr<parsing::Source> source;

  void construct();

public:
  SourceCache(std::shared_ptr<parsing::Source> source);
  std::optional<std::reference_wrapper<const Line>>
  getLine(std::size_t offset) const;
};

class ReportCache {
private:
  std::unordered_map<std::string_view, SourceCache> cache;

public:
  ReportCache() = default;

  void addSource(std::shared_ptr<parsing::Source> source);
  std::optional<std::reference_wrapper<const SourceCache>>
  getSource(std::string_view filename);
};

} // namespace ylang::report
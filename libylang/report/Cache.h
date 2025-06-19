#pragma once

#include "parsing/Manager.h"
#include <cstddef>
#include <memory>
#include <optional>
#include <span>
#include <unordered_map>
#include <vector>

namespace ylang::report {

struct StringHash {
  using is_transparent = void;
  std::size_t operator()(std::string_view v) const noexcept {
    return std::hash<std::string_view>{}(v);
  }
  std::size_t operator()(const std::string &s) const noexcept {
    return std::hash<std::string_view>{}(s);
  }
};
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
  [[nodiscard]] std::optional<std::reference_wrapper<const Line>>
  getLine(std::size_t offset) const;
  [[nodiscard]] std::span<const Line> linesSpan() const { return lines; }
};

class ReportCache {
private:
  std::unordered_map<std::string, SourceCache, StringHash, std::equal_to<>>
      cache;

public:
  ReportCache() = default;

  void addSource(std::shared_ptr<parsing::Source> source);
  [[nodiscard]]
  std::optional<std::reference_wrapper<const SourceCache>>
  getSource(std::string_view filename) const;
};

} // namespace ylang::report


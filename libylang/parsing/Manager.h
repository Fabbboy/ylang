#pragma once

#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace ylang::parsing {
using ContentId = std::size_t;

struct Source {
public:
  std::string content;
  std::string filename;

  Source(std::string_view content, std::string_view filename);
};

struct Manager {
private:
  std::vector<Source> contents;
  ContentId next_id;

public:
  Manager();

  ContentId add_content(Source &&source);
  std::optional<const Source *> get_content(ContentId id) const;
};
} // namespace ylang::parsing
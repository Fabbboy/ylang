#pragma once

#include <optional>
#include <string>
#include <vector>

namespace ylang::parser {
using ContentId = std::size_t;

struct Manager {
private:
  std::vector<std::string> contents;
  ContentId next_id;

public:
  Manager();

  ContentId add_content(std::string &&content);
  std::optional<const std::string *> get_content(ContentId id) const;
};
} // namespace ylang::parser
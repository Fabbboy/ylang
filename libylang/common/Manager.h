#pragma once

#include <memory>
#include <string>
#include <string_view>
#include <vector>

namespace ylang::common {

struct Source {
public:
  std::string content;
  std::string filename;

  Source(std::string_view content, std::string_view filename);
};

struct Manager {
private:
  std::vector<std::shared_ptr<Source>> contents;

public:
  Manager();

  std::shared_ptr<Source> addContent(std::string_view content,
                                     std::string_view filename);
};
} // namespace ylang::common
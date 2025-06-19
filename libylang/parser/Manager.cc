#include <optional>
#include <parser/Manager.h>

namespace ylang::parser {
Manager::Manager() : next_id(0) {};

ContentId Manager::add_content(std::string &&content) {
  contents.push_back(std::move(content));
  return next_id++;
}

std::optional<const std::string *> Manager::get_content(ContentId id) const {
  if (id < contents.size()) {
    return &contents[id];
  }
  return std::nullopt;
}

} // namespace ylang::parser
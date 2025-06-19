#include <optional>
#include <parsing/Manager.h>

namespace ylang::parsing {

Source::Source(std::string_view content, std::string_view filename)
    : content(content), filename(filename) {}

Manager::Manager() : next_id(0) {};

ContentId Manager::add_content(Source &&source) {
  contents.emplace_back(std::move(source));
  return next_id++;
}

std::optional<const Source *> Manager::get_content(ContentId id) const {
  if (id < next_id) {
    return &contents[id];
  }
  return std::nullopt;
}
} // namespace ylang::parsing
#include <common/Manager.h>
#include <memory>
#include <optional>
#include <string_view>

namespace sable::common {

Source::Source(std::string_view content, std::string_view filename)
    : content(content), filename(filename) {}

Manager::Manager() : contents() {}

std::shared_ptr<Source> Manager::addContent(std::string_view content,
                                            std::string_view filename) {
  std::shared_ptr<Source> source = std::make_shared<Source>(content, filename);
  contents.push_back(source);
  return source;
}

std::optional<std::shared_ptr<Source>> Manager::getContent(std::string_view filename) const {
  for (const auto &source : contents) {
    if (source->filename == filename) {
      return source;
    }
  }
  return nullptr;
}
} // namespace sable::common
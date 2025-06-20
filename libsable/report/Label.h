#pragma once

#include "common/Manager.h"
#include "report/Span.h"
#include <optional>
#include <ostream>
#include <string>

namespace sable::report {

class Label {
private:
  Span span;
  std::optional<std::string> message;

public:
  explicit Label(Span span);

  Label &withMessage(const std::string &msg);
  std::ostream &print(std::ostream &os, const common::Manager &manager) const;
};

} // namespace sable::report

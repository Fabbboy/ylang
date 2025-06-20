#pragma once

#include "report/Cache.h"
#include "report/Label.h"
#include "report/Span.h"
#include "report/Write.h"
#include <optional>
#include <ostream>
#include <string>
#include <vector>

#define ANSI_COLOR(color) "\033[" color "m"
#define ANSI_RESET ANSI_COLOR("0")
#define ANSI_RED ANSI_COLOR("31")
#define ANSI_YELLOW ANSI_COLOR("33")
#define ANSI_BLUE ANSI_COLOR("34")

namespace sable::report {

#define SEVERITY_LEVELS \
  X(Info, ANSI_BLUE) \
  X(Warning, ANSI_YELLOW) \
  X(Error, ANSI_RED)

enum class Severity {
#define X(name, color) name,
  SEVERITY_LEVELS
#undef X
};

std::ostream &operator<<(std::ostream &os, Severity severity);

class Diagnostic {
private:
  Severity severity;
  std::optional<std::string> message;
  std::optional<Span> code;
  std::vector<Label> labels;

public:
  explicit Diagnostic(Severity severity);

  Diagnostic &withMessage(const std::string &msg);
  Diagnostic &withCode(const Span &span);
  Diagnostic &withLabel(const Label &label);

  std::ostream &print(std::ostream &os, const Cache &cache) const;
};

} // namespace sable::report

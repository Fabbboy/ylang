#pragma once

#include <optional>
#include <ostream>
#include <string>
namespace ylang::report {
#define SEVERITY_LEVELS                                                        \
  X(Info)                                                                      \
  X(Warning)                                                                   \
  X(Error)

enum class Severity {
#define X(name) name,
  SEVERITY_LEVELS
#undef X
};
std::ostream &operator<<(std::ostream &os, Severity severity);

class Diagnostic {
private:
  Severity severity;
  std::optional<std::string> message;

public:
  Diagnostic(Severity severity);

  inline void withMessage(const std::string &msg) { message = msg; }

  std::string write() const;
  friend std::ostream &operator<<(std::ostream &os, const Diagnostic &diag);
};

} // namespace ylang::report
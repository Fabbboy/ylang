#pragma once

#include "parsing/Location.h"
#include <string>
#include <string_view>

namespace ylang::report {

enum class Severity { Note, Warning, Error };

class Diagnostic {
public:
  virtual ~Diagnostic() = default;
  virtual Severity severity() const = 0;
  virtual std::string_view message() const = 0;
  virtual parsing::Location location() const = 0;
};

class BasicDiagnostic : public Diagnostic {
private:
  Severity sev_;
  std::string msg_;
  parsing::Location loc_;

public:
  BasicDiagnostic(Severity sev, std::string msg, parsing::Location loc);

  Severity severity() const override { return sev_; }
  std::string_view message() const override { return msg_; }
  parsing::Location location() const override { return loc_; }
};

} // namespace ylang::report

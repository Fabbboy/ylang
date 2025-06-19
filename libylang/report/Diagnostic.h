#pragma once

#include "parsing/Location.h"
#include <optional>
#include <span>
#include <string>
#include <string_view>
#include <vector>

namespace ylang::report {

enum class Severity { Note, Warning, Error };

struct Label {
  parsing::Location loc;
  std::string message;
};

class Diagnostic {
public:
  virtual ~Diagnostic() = default;
  virtual Severity severity() const = 0;
  virtual std::optional<std::string_view> message() const = 0;
  virtual std::span<const Label> labels() const = 0;
};

class BasicDiagnostic : public Diagnostic {
private:
  Severity sev_;
  std::optional<std::string> msg_;
  std::vector<Label> labels_;

public:
  BasicDiagnostic(Severity sev, std::optional<std::string> msg,
                  std::vector<Label> labels);

  Severity severity() const override { return sev_; }
  std::optional<std::string_view> message() const override {
    if (msg_)
      return std::string_view(*msg_);
    return std::nullopt;
  }
  std::span<const Label> labels() const override { return labels_; }
};

} // namespace ylang::report


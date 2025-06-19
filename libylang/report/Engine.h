#pragma once

#include "report/Diagnostic.h"
#include <iostream>

namespace ylang::report {
class DiagnosticEngine {
public:
  virtual ~DiagnosticEngine() = default;

  virtual void report(const Diagnostic &diag) = 0;
};

class StreamWriter : public DiagnosticEngine {
private:
  std::ostream &os;

public:
  explicit StreamWriter(std::ostream &output_stream);

  void report(const Diagnostic &diag) override;
};

} // namespace ylang::report
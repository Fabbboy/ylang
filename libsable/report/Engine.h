#pragma once

#include "report/Diagnostic.h"
#include <iostream>

namespace sable::report {
template <typename S> class DiagnosticEngine {
public:
  virtual ~DiagnosticEngine() = default;

  virtual void report(const Diagnostic<S> &diag) = 0;
};

template <typename S> class StreamWriter : public DiagnosticEngine<S> {
private:
  std::ostream &os;

public:
  explicit StreamWriter(std::ostream &output_stream);

  void report(const Diagnostic<S> &diag) override {}
};

} // namespace sable::report
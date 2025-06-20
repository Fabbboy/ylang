#pragma once

#include "common/Manager.h"
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
  const common::Manager &manager;

public:
  explicit StreamWriter(std::ostream &output_stream,
                        const common::Manager &manager)
      : os(output_stream), manager(manager) {}

  void report(const Diagnostic<S> &diag) override { diag.print(os, manager); }
};

} // namespace sable::report
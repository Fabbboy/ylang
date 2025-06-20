#pragma once

#include "report/Cache.h"
#include "report/Diagnostic.h"
#include <iostream>
#include <ostream>

namespace sable::report {
template <typename S> class DiagnosticEngine {
public:
  virtual ~DiagnosticEngine() = default;

  virtual void report(const Diagnostic<S> &diag) = 0;
};

template <typename S> class StreamWriter : public DiagnosticEngine<S> {
private:
  std::ostream &os;
  const Cache<S> &cache;

public:
  explicit StreamWriter(std::ostream &output_stream, const Cache<S> &manager)
      : os(output_stream), cache(manager) {}

  void report(const Diagnostic<S> &diag) override {
    diag.print(os, cache);
    os << std::endl;
  }
};

} // namespace sable::report
#pragma once

#include "report/Cache.h"
#include "report/Diagnostic.h"
#include <ostream>

namespace sable::report {

class DiagnosticEngine {
public:
  virtual ~DiagnosticEngine() = default;
  virtual void report(const Diagnostic &diag) = 0;
};

class StreamWriter : public DiagnosticEngine {
private:
  std::ostream &os;
  const Cache &cache;

public:
  StreamWriter(std::ostream &output_stream, const Cache &manager);

  void report(const Diagnostic &diag) override;
};

} // namespace sable::report

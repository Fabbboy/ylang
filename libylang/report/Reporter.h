#pragma once

#include "Cache.h"
#include "Diagnostic.h"
#include <iosfwd>
#include <iostream>

namespace ylang::report {

class Reporter {
public:
  virtual ~Reporter() = default;
  virtual void report(const Diagnostic &diag) = 0;
};

class TextReporter : public Reporter {
private:
  ReportCache &cache_;
  std::ostream &os_;

public:
  TextReporter(ReportCache &cache, std::ostream &os = std::cerr);

  void report(const Diagnostic &diag) override;
};

} // namespace ylang::report

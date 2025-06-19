#pragma once

#include "Cache.h"
#include "Diagnostic.h"
#include <iostream>

namespace ylang::report {

class Reporter {
public:
  virtual ~Reporter() = default;
  virtual void report(const Diagnostic &diag) = 0;
};

class ConsoleReporter : public Reporter {
private:
  ReportCache &cache_;
  std::ostream &os_;
  void printHeader(const Diagnostic &diag);
  void printLabel(const Label &label);
  void printSnippet(const Label &label, const SourceCache &src);

public:
  ConsoleReporter(ReportCache &cache, std::ostream &os = std::cerr);

  void report(const Diagnostic &diag) override;
};

} // namespace ylang::report


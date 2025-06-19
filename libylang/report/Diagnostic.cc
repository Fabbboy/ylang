#include <report/Diagnostic.h>

namespace ylang::report {

BasicDiagnostic::BasicDiagnostic(Severity sev, std::string msg, parsing::Location loc)
    : sev_(sev), msg_(std::move(msg)), loc_(std::move(loc)) {}

} // namespace ylang::report

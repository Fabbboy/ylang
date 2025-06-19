#include <report/Diagnostic.h>

namespace ylang::report {

BasicDiagnostic::BasicDiagnostic(Severity sev, std::string msg,
                                 std::vector<Label> labels)
    : sev_(sev), msg_(std::move(msg)), labels_(std::move(labels)) {}

} // namespace ylang::report


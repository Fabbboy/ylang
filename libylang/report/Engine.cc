#include <report/Engine.h>

namespace ylang::report {
StreamWriter::StreamWriter(std::ostream &output_stream) : os(output_stream) {}
void StreamWriter::report(const Diagnostic &diag) { os << diag << std::endl; }
} // namespace ylang::report
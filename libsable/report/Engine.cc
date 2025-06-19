#include <report/Engine.h>

namespace sable::report {
StreamWriter::StreamWriter(std::ostream &output_stream) : os(output_stream) {}
void StreamWriter::report(const Diagnostic &diag) { os << diag << std::endl; }
} // namespace sable::report
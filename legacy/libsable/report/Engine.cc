#include "report/Engine.h"

namespace sable::report {

StreamWriter::StreamWriter(std::ostream &output_stream, const Cache &manager)
    : os(output_stream), cache(manager) {}

void StreamWriter::report(const Diagnostic &diag) { diag.print(os, cache); }

} // namespace sable::report

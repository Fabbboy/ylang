#include <iomanip>
#include <iostream>
#include <report/Reporter.h>

namespace ylang::report {

static const char *severityToString(Severity s) {
  switch (s) {
  case Severity::Note:
    return "note";
  case Severity::Warning:
    return "warning";
  case Severity::Error:
    return "error";
  }
  return "";
}

TextReporter::TextReporter(ReportCache &cache, std::ostream &os)
    : cache_(cache), os_(os) {}

void TextReporter::report(const Diagnostic &diag) {
  os_ << severityToString(diag.severity()) << ": " << diag.message() << '\n';
  auto loc = diag.location();
  if (!loc.file) {
    return;
  }
  auto srcOpt = cache_.getSource(loc.file->filename);
  if (!srcOpt) {
    os_ << " -> " << loc.file->filename << ':' << loc.start + 1 << '\n';
    return;
  }
  const auto &srcCache = srcOpt->get();
  auto lineOpt = srcCache.getLine(loc.start);
  if (!lineOpt) {
    os_ << " -> " << loc.file->filename << ':' << loc.start + 1 << '\n';
    return;
  }
  const Line &line = lineOpt->get();
  std::string_view lineView(loc.file->content.data() + line.start,
                            line.stop - line.start);
  std::size_t colStart = loc.start - line.start;
  std::size_t colEnd = loc.stop - line.start;
  os_ << " --> " << loc.file->filename << ':' << line.line << ':' << colStart + 1
      << '\n';
  std::string lineNumStr = std::to_string(line.line);
  os_ << lineNumStr << " | " << lineView << '\n';
  os_ << std::string(lineNumStr.size(), ' ') << " | "
      << std::string(colStart, ' ')
      << std::string(std::max<std::size_t>(1, colEnd - colStart), '^') << '\n';
}

} // namespace ylang::report

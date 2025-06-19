#include <algorithm>
#include <functional>
#include <ostream>
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
  for (const auto &label : diag.labels()) {
    auto loc = label.loc;
    if (!loc.file) {
      continue;
    }
    auto srcOpt = cache_.getSource(loc.file->filename);
    if (!srcOpt) {
      os_ << " -> " << loc.file->filename << ':' << loc.start + 1 << '\n';
      continue;
    }
    const auto &srcCache = srcOpt->get();
    auto startLineOpt = srcCache.getLine(loc.start);
    auto endLineOpt = srcCache.getLine(loc.stop ? loc.stop - 1 : loc.stop);
    if (!startLineOpt || !endLineOpt) {
      os_ << " -> " << loc.file->filename << ':' << loc.start + 1 << '\n';
      continue;
    }
    const auto &source = loc.file->content;
    auto linesSpan = srcCache.linesSpan();
    std::size_t startIdx = startLineOpt->get().line - 1;
    std::size_t endIdx = endLineOpt->get().line - 1;
    for (std::size_t idx = startIdx; idx <= endIdx; ++idx) {
      const Line &line = linesSpan[idx];
      std::string_view lineView(source.data() + line.start,
                               line.stop - line.start);
      std::size_t colStart = (idx == startIdx) ? loc.start - line.start : 0;
      std::size_t colEnd =
          (idx == endIdx) ? loc.stop - line.start : line.stop - line.start;

      constexpr std::size_t WIDTH = 80;
      std::size_t cut = 0;
      if (lineView.size() > WIDTH) {
        if (colEnd > WIDTH) {
          cut = (colStart > 40) ? colStart - 40 : 0;
          if (cut + WIDTH > lineView.size())
            cut = lineView.size() - WIDTH;
        }
        lineView = lineView.substr(cut, std::min(WIDTH, lineView.size() - cut));
        colStart -= cut;
        colEnd -= cut;
      }

      os_ << " --> " << loc.file->filename << ':' << line.line << ':'
          << colStart + 1 << '\n';
      std::string lineNumStr = std::to_string(line.line);
      os_ << lineNumStr << " | " << lineView << '\n';
      os_ << std::string(lineNumStr.size(), ' ') << " | "
          << std::string(colStart, ' ')
          << std::string(std::max<std::size_t>(1, colEnd - colStart), '^')
          << '\n';
    }
    if (!label.message.empty()) {
      os_ << "     " << label.message << '\n';
    }
  }
}

} // namespace ylang::report


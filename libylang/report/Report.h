#pragma once

#include "parsing/Lexer/Lexer.h"
#include "parsing/Manager.h"
#include <string>
#include <string_view>
namespace ylang::parsing {
struct Report {
public:
  virtual ~Report() = default;

  virtual std::string write(std::string_view file_name,
                            std::string_view source_slice) const = 0;

  virtual Location get_location() const = 0;
};

} // namespace ylang::parsing
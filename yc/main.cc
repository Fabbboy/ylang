#include "parsing/Manager.h"
#include <iostream>
#include <string_view>

std::string_view SOURCE = R"(

)";

int main() {
  ylang::parsing::Manager manager;
  ylang::parsing::Source source(SOURCE, "main.y");
  ylang::parsing::ContentId id = manager.add_content(std::move(source));

  auto content = manager.get_content(id);
  if (content) {
    std::cout << "Content ID: " << id << "\n";
    std::cout << "Filename: " << (*content)->filename << "\n";
    std::cout << "Content: " << (*content)->content << "\n";
  } else {
    std::cout << "Content not found for ID: " << id << "\n";
  }
}
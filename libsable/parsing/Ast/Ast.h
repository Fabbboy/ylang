#pragma once

namespace sable::parsing {
struct Node {
public:
  virtual ~Node() = default;

  virtual void accept(class AstVisitor &visitor) = 0;
};

class Ast {
public:
  Ast() = default;
};
} // namespace sable::parsing
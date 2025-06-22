pub trait Node {
    fn accept(&self, visitor: &mut dyn AstVisitor);
}

pub trait AstVisitor {}

pub struct Ast;

impl Ast {
    pub fn new() -> Self { Self }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct DummyVisitor;
    impl AstVisitor for DummyVisitor {}
    struct DummyNode;
    impl Node for DummyNode {
        fn accept(&self, _visitor: &mut dyn AstVisitor) {}
    }
    #[test]
    fn ast_basic() {
        let _ast = Ast::new();
    }
}

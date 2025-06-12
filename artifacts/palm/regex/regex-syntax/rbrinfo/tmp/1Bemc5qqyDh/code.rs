pub fn visit<V: Visitor>(ast: &Ast, visitor: V) -> Result<V::Output, V::Err> {
    HeapVisitor::new().visit(ast, visitor)
}
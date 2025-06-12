pub fn visit<V: Visitor>(hir: &Hir, visitor: V) -> Result<V::Output, V::Err> {
    HeapVisitor::new().visit(hir, visitor)
}
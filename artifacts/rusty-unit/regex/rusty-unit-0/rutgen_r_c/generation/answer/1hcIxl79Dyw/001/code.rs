// Answer 0

#[test]
fn test_heap_visitor_new() {
    let visitor = HeapVisitor::new();
    assert_eq!(visitor.stack.len(), 0);
    assert_eq!(visitor.stack, Vec::<(&Hir, Frame)>::new());
}


// Answer 0

#[test]
fn test_heap_visitor_new() {
    struct DummyAst;

    let visitor: HeapVisitor<'_> = HeapVisitor::new();
    assert_eq!(visitor.stack.len(), 0);
    assert_eq!(visitor.stack_class.len(), 0);
}

#[test]
fn test_heap_visitor_new_with_dummy_structure() {
    struct DummyAst;

    let visitor: HeapVisitor<'_> = HeapVisitor::new();
    assert_eq!(visitor.stack.len(), 0);
    assert_eq!(visitor.stack_class.len(), 0);
}


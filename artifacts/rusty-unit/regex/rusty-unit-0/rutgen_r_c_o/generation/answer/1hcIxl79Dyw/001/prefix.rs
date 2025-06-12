// Answer 0

#[test]
fn test_new_heap_visitor_empty_stack() {
    let visitor = HeapVisitor::new();
}

#[test]
fn test_new_heap_visitor_initialization() {
    let visitor = HeapVisitor::new();
    let expected_stack: Vec<(&Hir, Frame)> = vec![];
    assert_eq!(visitor.stack, expected_stack);
}


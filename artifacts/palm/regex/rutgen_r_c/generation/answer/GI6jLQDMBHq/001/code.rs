// Answer 0

#[test]
fn test_heap_visitor_new() {
    // Initialize the HeapVisitor using its new method
    let visitor: HeapVisitor = HeapVisitor::new();

    // Check that the stack is empty
    assert!(visitor.stack.is_empty());

    // Check that the stack_class is also empty
    assert!(visitor.stack_class.is_empty());
}


// Answer 0

#[test]
fn test_heap_visitor_new() {
    struct HeapVisitor<'a> {
        stack: Vec<&'a str>,
    }

    let visitor: HeapVisitor = HeapVisitor::new();
    
    assert_eq!(visitor.stack.len(), 0);
}


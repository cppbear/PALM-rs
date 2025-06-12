// Answer 0

#[test]
fn test_new_heap_visitor() {
    struct HeapVisitor<'a> {
        stack: Vec<&'a str>,
        stack_class: Vec<&'a str>,
    }

    impl<'a> HeapVisitor<'a> {
        fn new() -> HeapVisitor<'a> {
            HeapVisitor { stack: vec![], stack_class: vec![] }
        }
    }

    let visitor = HeapVisitor::new();
    assert_eq!(visitor.stack.len(), 0);
    assert_eq!(visitor.stack_class.len(), 0);
}


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

    let visitor: HeapVisitor = HeapVisitor::new();
    assert_eq!(visitor.stack, vec![]);
    assert_eq!(visitor.stack_class, vec![]);
}

#[test]
fn test_new_heap_visitor_empty() {
    struct HeapVisitor<'a> {
        stack: Vec<&'a str>,
        stack_class: Vec<&'a str>,
    }

    impl<'a> HeapVisitor<'a> {
        fn new() -> HeapVisitor<'a> {
            HeapVisitor { stack: vec![], stack_class: vec![] }
        }
    }

    let visitor: HeapVisitor = HeapVisitor::new();
    assert!(visitor.stack.is_empty());
    assert!(visitor.stack_class.is_empty());
}


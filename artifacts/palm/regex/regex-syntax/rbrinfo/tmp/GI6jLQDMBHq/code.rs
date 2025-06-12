fn new() -> HeapVisitor<'a> {
        HeapVisitor { stack: vec![], stack_class: vec![] }
    }
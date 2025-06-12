// Answer 0

#[test]
fn test_pop_class_union_not_empty() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        // Implement the required methods for the Visitor trait here
    }

    let item1 = ClassSetItem::Literal(Literal::new("a"));
    let item2 = ClassSetItem::Literal(Literal::new("b"));

    let tail = vec![item2.clone()];
    let induct = ClassFrame::Union {
        head: &item1,
        tail: &tail,
    };

    let visitor = &mut TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.pop_class(induct);

    match result {
        Some(ClassFrame::Union { head, tail }) => {
            assert_eq!(head, &item1);
            assert_eq!(tail.len(), 1);
            assert_eq!(tail[0], item2);
        },
        _ => panic!("Expected Some(ClassFrame::Union)"),
    }
}

#[test]
fn test_pop_class_union_empty_tail() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        // Implement the required methods for the Visitor trait here
    }

    let item = ClassSetItem::Literal(Literal::new("a"));
    let tail: Vec<ClassSetItem> = vec![];
    let induct = ClassFrame::Union {
        head: &item,
        tail: &tail,
    };

    let visitor = &mut TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.pop_class(induct);

    assert!(result.is_none(), "Expected None when tail is empty");
}

#[test]
fn test_pop_class_binary_lhs() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        // Implement the required methods for the Visitor trait here
    }

    let op = ClassSetBinaryOp {
        span: Span::new(0, 1),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new("a")))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new("b")))),
    };

    let induct = ClassFrame::BinaryLHS {
        op: &op,
        lhs: &ClassSet::Item(ClassSetItem::Literal(Literal::new("a"))),
        rhs: &ClassSet::Item(ClassSetItem::Literal(Literal::new("b"))),
    };

    let visitor = &mut TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.pop_class(induct);

    match result {
        Some(ClassFrame::BinaryRHS { op: _, rhs }) => {
            assert_eq!(rhs, &ClassSet::Item(ClassSetItem::Literal(Literal::new("b"))));
        },
        _ => panic!("Expected Some(ClassFrame::BinaryRHS)"),
    }
}

#[test]
fn test_pop_class_binary_rhs() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        // Implement the required methods for the Visitor trait here
    }

    let op = ClassSetBinaryOp {
        span: Span::new(0, 1),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new("a")))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new("b")))),
    };

    let induct = ClassFrame::BinaryRHS {
        op: &op,
        rhs: &ClassSet::Item(ClassSetItem::Literal(Literal::new("b"))),
    };

    let visitor = &mut TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.pop_class(induct);

    assert!(result.is_none(), "Expected None for BinaryRHS");
}


// Answer 0

#[test]
fn test_pop_class_binary_frame() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let visitor = DummyVisitor;

    let binary_op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::default(),
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::default()))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::default()))),
    };
    
    let induct_frame = ClassFrame::Binary { op: &binary_op };

    let heap_visitor = HeapVisitor::new();

    assert_eq!(heap_visitor.pop_class(induct_frame), None);
}


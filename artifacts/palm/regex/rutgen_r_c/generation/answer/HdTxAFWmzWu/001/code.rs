// Answer 0

#[test]
fn test_pop_class_binary_rhs() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn on_ast(&mut self, _ast: &Ast) {}
    }

    let op = ClassSetBinaryOp {
        span: Span { start: 0, end: 1 },
        kind: ClassSetBinaryOpKind::And,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::One))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::Two))),
    };

    let induct = ClassFrame::BinaryRHS { op: &op };

    let visitor = TestVisitor;
    let visitor_mut = &mut visitor;

    let heap_visitor = HeapVisitor::new();
    let result = heap_visitor.pop_class(induct);
    
    assert_eq!(result, None);
}


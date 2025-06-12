// Answer 0

#[test]
fn test_child_binary_lhs_item() {
    struct DummySpan;
    struct DummyLiteral;
    struct DummyClassSet;
    
    let lhs = Box::new(ClassSet::Item(ClassSetItem::Literal(DummyLiteral)));
    let rhs = Box::new(ClassSet::Item(ClassSetItem::Empty(DummySpan)));

    let binary_op = ClassSetBinaryOp {
        span: DummySpan,
        kind: ClassSetBinaryOpKind, // assuming this can be instantiated
        lhs: lhs.clone(),
        rhs: rhs.clone(),
    };

    let frame = ClassFrame::BinaryLHS { op: &binary_op, lhs: &*lhs, rhs: &*rhs };
    let induct = frame.child();

    match induct {
        ClassInduct::Item(item) => match item {
            ClassSetItem::Literal(_) => assert!(true),
            _ => assert!(false, "Expected a Literal item"),
        },
        _ => assert!(false, "Expected an Item induct type"),
    }
}

#[test]
fn test_child_binary_rhs_item() {
    struct DummySpan;
    struct DummyLiteral;
    struct DummyClassSet;

    let lhs = Box::new(ClassSet::Item(ClassSetItem::Empty(DummySpan)));
    let rhs = Box::new(ClassSet::Item(ClassSetItem::Literal(DummyLiteral)));

    let binary_op = ClassSetBinaryOp {
        span: DummySpan,
        kind: ClassSetBinaryOpKind, // assuming this can be instantiated
        lhs: lhs.clone(),
        rhs: rhs.clone(),
    };

    let frame = ClassFrame::BinaryRHS { op: &binary_op, rhs: &*rhs };
    let induct = frame.child();

    match induct {
        ClassInduct::Item(item) => match item {
            ClassSetItem::Literal(_) => assert!(true),
            _ => assert!(false, "Expected a Literal item"),
        },
        _ => assert!(false, "Expected an Item induct type"),
    }
}


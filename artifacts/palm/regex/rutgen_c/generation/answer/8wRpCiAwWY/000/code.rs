// Answer 0

#[test]
fn test_child_union_variant() {
    struct DummyClassSetItem;
    let item = &DummyClassSetItem;
    let frame = ClassFrame::Union { head: item, tail: &[] };
    let induct = frame.child();
    match induct {
        ClassInduct::Item(head) => assert_eq!(head, item),
        _ => panic!("Expected ClassInduct::Item"),
    }
}

#[test]
fn test_child_binary_variant() {
    struct DummyClassSetBinaryOp;
    let op = &DummyClassSetBinaryOp;
    let frame = ClassFrame::Binary { op };
    let induct = frame.child();
    match induct {
        ClassInduct::BinaryOp(op_out) => assert_eq!(op_out, op),
        _ => panic!("Expected ClassInduct::BinaryOp"),
    }
}

#[test]
fn test_child_binary_lhs_variant() {
    struct DummyClassSet;
    let lhs = &DummyClassSet;
    let rhs = &DummyClassSet;
    let op = &ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::default(),
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    };
    let frame = ClassFrame::BinaryLHS { op, lhs, rhs };
    let induct = frame.child();
    match induct {
        ClassInduct::Item(item) => assert_eq!(item, lhs),
        _ => panic!("Expected ClassInduct::Item from lhs"),
    }
}

#[test]
fn test_child_binary_rhs_variant() {
    struct DummyClassSet;
    let lhs = &DummyClassSet;
    let rhs = &DummyClassSet;
    let op = &ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::default(),
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    };
    let frame = ClassFrame::BinaryRHS { op, rhs };
    let induct = frame.child();
    match induct {
        ClassInduct::Item(item) => assert_eq!(item, rhs),
        _ => panic!("Expected ClassInduct::Item from rhs"),
    }
}


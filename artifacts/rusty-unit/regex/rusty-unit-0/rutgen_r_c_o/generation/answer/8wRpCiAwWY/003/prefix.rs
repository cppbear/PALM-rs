// Answer 0

#[test]
fn test_child_binary_op_1() {
    let op = ClassSetBinaryOp { 
        span: Span { /* initialize with valid data */ }, 
        kind: ClassSetBinaryOpKind::SomeKind, 
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal { /* initialize with valid data */ }))), 
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal { /* initialize with valid data */ }))) 
    };
    let frame = ClassFrame::Binary { op: &op };
    let _ = frame.child();
}

#[test]
fn test_child_binary_op_50() {
    let op = ClassSetBinaryOp { 
        span: Span { /* initialize with valid data */ }, 
        kind: ClassSetBinaryOpKind::SomeKind, 
        lhs: Box::new(ClassSet::Item(ClassSetItem::Range(ClassSetRange { /* initialize with valid data */ }))), 
        rhs: Box::new(ClassSet::Item(ClassSetItem::Range(ClassSetRange { /* initialize with valid data */ }))) 
    };
    let frame = ClassFrame::Binary { op: &op };
    let _ = frame.child();
}

#[test]
fn test_child_binary_op_100() {
    let op = ClassSetBinaryOp { 
        span: Span { /* initialize with valid data */ }, 
        kind: ClassSetBinaryOpKind::SomeKind, 
        lhs: Box::new(ClassSet::Item(ClassSetItem::Unicode(ClassUnicode { /* initialize with valid data */ }))), 
        rhs: Box::new(ClassSet::Item(ClassSetItem::Unicode(ClassUnicode { /* initialize with valid data */ }))) 
    };
    let frame = ClassFrame::Binary { op: &op };
    let _ = frame.child();
}

#[test]
fn test_child_binary_op_empty_lhs_rhs() {
    let op = ClassSetBinaryOp { 
        span: Span { /* initialize with valid data */ }, 
        kind: ClassSetBinaryOpKind::SomeKind, 
        lhs: Box::new(ClassSet::Item(ClassSetItem::Empty(Span { /* initialize with valid data */ }))), 
        rhs: Box::new(ClassSet::Item(ClassSetItem::Empty(Span { /* initialize with valid data */ }))) 
    };
    let frame = ClassFrame::Binary { op: &op };
    let _ = frame.child();
}


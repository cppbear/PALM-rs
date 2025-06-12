// Answer 0

#[test]
fn test_class_induct_binary_op_difference_with_empty_item() {
    let lhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Empty(Span { start: 0, end: 0 })],
    });
    let rhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Empty(Span { start: 1, end: 1 })],
    });
    let binary_op = ClassInduct::BinaryOp(ClassSetBinaryOp {
        span: Span { start: 0, end: 1 },
        kind: ClassSetBinaryOpKind::Difference,
        lhs,
        rhs,
    });
    let _ = format!("{:?}", binary_op);
}

#[test]
fn test_class_induct_binary_op_difference_with_literal() {
    let lhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Literal(Literal::new('a'))],
    });
    let rhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Literal(Literal::new('b'))],
    });
    let binary_op = ClassInduct::BinaryOp(ClassSetBinaryOp {
        span: Span { start: 0, end: 2 },
        kind: ClassSetBinaryOpKind::Difference,
        lhs,
        rhs,
    });
    let _ = format!("{:?}", binary_op);
}

#[test]
fn test_class_induct_binary_op_difference_with_range() {
    let lhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Range(ClassSetRange::new('a', 'c'))],
    });
    let rhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Range(ClassSetRange::new('b', 'd'))],
    });
    let binary_op = ClassInduct::BinaryOp(ClassSetBinaryOp {
        span: Span { start: 0, end: 3 },
        kind: ClassSetBinaryOpKind::Difference,
        lhs,
        rhs,
    });
    let _ = format!("{:?}", binary_op);
}

#[test]
fn test_class_induct_binary_op_difference_with_ascii() {
    let lhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Ascii(ClassAscii::new("[:alnum:]"))],
    });
    let rhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Ascii(ClassAscii::new("[:alpha:]"))],
    });
    let binary_op = ClassInduct::BinaryOp(ClassSetBinaryOp {
        span: Span { start: 0, end: 5 },
        kind: ClassSetBinaryOpKind::Difference,
        lhs,
        rhs,
    });
    let _ = format!("{:?}", binary_op);
}

#[test]
fn test_class_induct_binary_op_difference_with_union() {
    let lhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Union(ClassSetUnion::new(vec![ClassSetItem::Literal(Literal::new('x'))]))],
    });
    let rhs = Box::new(ClassSet {
        items: vec![ClassSetItem::Union(ClassSetUnion::new(vec![ClassSetItem::Literal(Literal::new('y'))]))],
    });
    let binary_op = ClassInduct::BinaryOp(ClassSetBinaryOp {
        span: Span { start: 0, end: 6 },
        kind: ClassSetBinaryOpKind::Difference,
        lhs,
        rhs,
    });
    let _ = format!("{:?}", binary_op);
}


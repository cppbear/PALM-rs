// Answer 0

#[test]
fn test_child_binary_rhs_with_literal() {
    let literal = ClassSetItem::Literal(Literal::new("a"));
    let rhs = ClassSet::Item(literal.clone());
    let lhs = ClassSet::BinaryOp(ClassSetBinaryOp {
        span: Span::new(1, 2),
        kind: ClassSetBinaryOpKind::SomeKind,
        lhs: Box::new(rhs.clone()),
        rhs: Box::new(lhs.clone()),
    });

    let frame = ClassFrame::BinaryRHS {
        op: &ClassSetBinaryOp {
            span: Span::new(1, 3),
            kind: ClassSetBinaryOpKind::SomeKind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        rhs: &rhs,
    };
    let _ = frame.child();
}

#[test]
fn test_child_binary_rhs_with_range() {
    let range = ClassSetItem::Range(ClassSetRange::new('a', 'z'));
    let rhs = ClassSet::Item(range.clone());
    let lhs = ClassSet::BinaryOp(ClassSetBinaryOp {
        span: Span::new(2, 4),
        kind: ClassSetBinaryOpKind::SomeKind,
        lhs: Box::new(rhs.clone()),
        rhs: Box::new(lhs.clone()),
    });

    let frame = ClassFrame::BinaryRHS {
        op: &ClassSetBinaryOp {
            span: Span::new(2, 5),
            kind: ClassSetBinaryOpKind::SomeKind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        rhs: &rhs,
    };
    let _ = frame.child();
}

#[test]
fn test_child_binary_rhs_with_union() {
    let union_item1 = ClassSetItem::Literal(Literal::new("a"));
    let union_item2 = ClassSetItem::Literal(Literal::new("b"));
    let union = ClassSetItem::Union(ClassSetUnion::new(vec![union_item1, union_item2]));
    let rhs = ClassSet::Item(union.clone());
    let lhs = ClassSet::BinaryOp(ClassSetBinaryOp {
        span: Span::new(3, 5),
        kind: ClassSetBinaryOpKind::SomeKind,
        lhs: Box::new(rhs.clone()),
        rhs: Box::new(lhs.clone()),
    });

    let frame = ClassFrame::BinaryRHS {
        op: &ClassSetBinaryOp {
            span: Span::new(3, 6),
            kind: ClassSetBinaryOpKind::SomeKind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        rhs: &rhs,
    };
    let _ = frame.child();
}

#[test]
fn test_child_binary_rhs_with_empty_union() {
    let empty_union = ClassSetItem::Union(ClassSetUnion::new(vec![]));
    let rhs = ClassSet::Item(empty_union.clone());
    let lhs = ClassSet::BinaryOp(ClassSetBinaryOp {
        span: Span::new(0, 1),
        kind: ClassSetBinaryOpKind::SomeKind,
        lhs: Box::new(rhs.clone()),
        rhs: Box::new(lhs.clone()),
    });

    let frame = ClassFrame::BinaryRHS {
        op: &ClassSetBinaryOp {
            span: Span::new(0, 2),
            kind: ClassSetBinaryOpKind::SomeKind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        rhs: &rhs,
    };
    let _ = frame.child();
}

#[test]
fn test_child_binary_rhs_zero_length_span() {
    let literal = ClassSetItem::Literal(Literal::new("a"));
    let rhs = ClassSet::Item(literal.clone());
    let lhs = ClassSet::BinaryOp(ClassSetBinaryOp {
        span: Span::new(0, 0),
        kind: ClassSetBinaryOpKind::SomeKind,
        lhs: Box::new(rhs.clone()),
        rhs: Box::new(lhs.clone()),
    });

    let frame = ClassFrame::BinaryRHS {
        op: &ClassSetBinaryOp {
            span: Span::new(0, 1),
            kind: ClassSetBinaryOpKind::SomeKind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        rhs: &rhs,
    };
    let _ = frame.child();
}


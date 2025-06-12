// Answer 0

#[test]
fn test_child_binary_lhs() {
    let span = Span::new(0, 10);
    let lhs = ClassSet::BinaryOp(ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('a')))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('b')))),
    });
    
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal::new('c')));

    let frame = ClassFrame::BinaryLHS {
        op: &ClassSetBinaryOp {
            span,
            kind: ClassSetBinaryOpKind::Intersection,
            lhs: Box::new(lhs.clone()),
            rhs: Box::new(rhs.clone()),
        },
        lhs: &lhs,
        rhs: &rhs,
    };

    let result = frame.child();
}

#[test]
fn test_child_binary_lhs_empty() {
    let span = Span::new(0, 10);
    let lhs = ClassSet::BinaryOp(ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Empty(span))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('b')))),
    });
    
    let rhs = ClassSet::Item(ClassSetItem::Empty(span));

    let frame = ClassFrame::BinaryLHS {
        op: &ClassSetBinaryOp {
            span,
            kind: ClassSetBinaryOpKind::Intersection,
            lhs: Box::new(lhs.clone()),
            rhs: Box::new(rhs.clone()),
        },
        lhs: &lhs,
        rhs: &rhs,
    };

    let result = frame.child();
}

#[test]
fn test_child_binary_lhs_complex() {
    let span = Span::new(0, 10);
    let lhs = ClassSet::BinaryOp(ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Range(ClassSetRange::new('a', 'z')))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('b')))),
    });
    
    let rhs = ClassSet::Item(ClassSetItem::Ascii(ClassAscii::Alnum));

    let frame = ClassFrame::BinaryLHS {
        op: &ClassSetBinaryOp {
            span,
            kind: ClassSetBinaryOpKind::Union,
            lhs: Box::new(lhs.clone()),
            rhs: Box::new(rhs.clone()),
        },
        lhs: &lhs,
        rhs: &rhs,
    };

    let result = frame.child();
}


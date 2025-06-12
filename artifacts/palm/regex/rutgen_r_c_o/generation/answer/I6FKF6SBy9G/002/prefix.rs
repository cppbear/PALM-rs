// Answer 0

#[test]
fn test_class_frame_binary_lhs() {
    let op = ClassSetBinaryOp {
        span: Span { start: 0, end: 1 },
        kind: ClassSetBinaryOpKind::Subtract,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::Character('a')))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::Character('z')))),
    };
    
    let lhs = &ClassSet::Item(ClassSetItem::Literal(Literal::Character('a')));
    let rhs = &ClassSet::Item(ClassSetItem::Literal(Literal::Character('z')));
    
    let frame = ClassFrame::BinaryLHS { op: &op, lhs, rhs };

    let _ = format!("{:?}", frame);
}

#[test]
fn test_class_frame_binary_lhs_with_empty() {
    let op = ClassSetBinaryOp {
        span: Span { start: 0, end: 1 },
        kind: ClassSetBinaryOpKind::Subtract,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Empty(Span { start: 0, end: 1 }))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Empty(Span { start: 0, end: 1 }))),
    };

    let lhs = &ClassSet::Item(ClassSetItem::Empty(Span { start: 0, end: 1 }));
    let rhs = &ClassSet::Item(ClassSetItem::Empty(Span { start: 0, end: 1 }));
    
    let frame = ClassFrame::BinaryLHS { op: &op, lhs, rhs };

    let _ = format!("{:?}", frame);
}

#[test]
fn test_class_frame_binary_lhs_with_union() {
    let op = ClassSetBinaryOp {
        span: Span { start: 0, end: 1 },
        kind: ClassSetBinaryOpKind::Subtract,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Union(ClassSetUnion { items: vec![] }))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Union(ClassSetUnion { items: vec![] }))),
    };

    let lhs = &ClassSet::Item(ClassSetItem::Union(ClassSetUnion { items: vec![] }));
    let rhs = &ClassSet::Item(ClassSetItem::Union(ClassSetUnion { items: vec![] }));
    
    let frame = ClassFrame::BinaryLHS { op: &op, lhs, rhs };

    let _ = format!("{:?}", frame);
}


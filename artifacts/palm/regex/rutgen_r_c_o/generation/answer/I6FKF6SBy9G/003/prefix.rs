// Answer 0

#[test]
fn test_class_frame_binary() {
    use ast::{ClassSetBinaryOp, ClassSet, ClassSetBinaryOpKind, Span};

    let span = Span { start: 0, end: 5 }; // Example Span
    let kind = ClassSetBinaryOpKind::Intersection; // Example Kind
    let lhs = ClassSet::Item(ClassSetItem::Literal(Literal::from('a'))); // Example LHS
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal::from('b'))); // Example RHS

    let binary_op = ClassSetBinaryOp {
        span,
        kind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let frame = ClassFrame::Binary { op: &binary_op };
    let _ = format!("{:?}", frame);
}

#[test]
fn test_class_frame_binary_with_empty_op() {
    use ast::{ClassSetBinaryOp, ClassSet, ClassSetBinaryOpKind, Span};

    let span = Span { start: 1, end: 3 }; // Example Span
    let kind = ClassSetBinaryOpKind::Union; // Example Kind
    let lhs = ClassSet::Item(ClassSetItem::Empty(span)); // Example LHS as Empty
    let rhs = ClassSet::Item(ClassSetItem::Empty(span)); // Example RHS as Empty

    let binary_op = ClassSetBinaryOp {
        span,
        kind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let frame = ClassFrame::Binary { op: &binary_op };
    let _ = format!("{:?}", frame);
}

#[test]
fn test_class_frame_binary_with_complex_op() {
    use ast::{ClassSetBinaryOp, ClassSet, ClassSetBinaryOpKind, Span};

    let span = Span { start: 2, end: 6 }; // Example Span
    let kind = ClassSetBinaryOpKind::Subtraction; // Example Kind
    let lhs = ClassSet::Item(ClassSetItem::Range(ClassSetRange(Start, End))); // Example LHS as Range
    let rhs = ClassSet::Item(ClassSetItem::Ascii(ClassAscii::Alnum)); // Example RHS as ASCII Class

    let binary_op = ClassSetBinaryOp {
        span,
        kind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let frame = ClassFrame::Binary { op: &binary_op };
    let _ = format!("{:?}", frame);
}


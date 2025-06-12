// Answer 0

#[test]
fn test_pop_class_binary_frame() {
    use ast::{ClassSetBinaryOp, ClassSet, ClassSetItem, Literal, Span};

    let span = Span { start: 0, end: 1 };
    let lhs = ClassSet::Item(ClassSetItem::Literal(Literal::Char('a')));
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal::Char('b')));
    let op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let induct = ClassFrame::Binary { op: &op };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);
}

#[test]
fn test_pop_class_binary_frame_edge_case_empty() {
    use ast::{ClassSetBinaryOp, ClassSet, ClassSetItem, Literal, Span};

    let span = Span { start: 0, end: 1 };
    let lhs = ClassSet::Item(ClassSetItem::Literal(Literal::Char('a')));
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal::Char('b')));
    let op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let induct = ClassFrame::Binary { op: &op };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);
}


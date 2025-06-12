// Answer 0

#[test]
fn test_span_binary_op_valid() {
    let span = Span { start: Position(0), end: Position(10) };
    let lhs = ClassSet::Item(ClassSetItem::Literal(Literal { span: Span { start: Position(0), end: Position(5) } }));
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal { span: Span { start: Position(5), end: Position(10) } }));
    let binary_op = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs: Box::new(lhs), rhs: Box::new(rhs) };
    let class_set = ClassSet::BinaryOp(binary_op);
    let _result = class_set.span();
}

#[test]
fn test_span_binary_op_edge_case() {
    let span = Span { start: Position(0), end: Position(0) };
    let lhs = ClassSet::Item(ClassSetItem::Empty(span.clone()));
    let rhs = ClassSet::Item(ClassSetItem::Empty(span.clone()));
    let binary_op = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Intersection, lhs: Box::new(lhs), rhs: Box::new(rhs) };
    let class_set = ClassSet::BinaryOp(binary_op);
    let _result = class_set.span();
}

#[test]
fn test_span_binary_op_empty() {
    let span = Span { start: Position(0), end: Position(10) };
    let lhs = ClassSet::Item(ClassSetItem::Empty(span.clone()));
    let rhs = ClassSet::Item(ClassSetItem::Union(ClassSetUnion::new(vec![])));
    let binary_op = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Difference, lhs: Box::new(lhs), rhs: Box::new(rhs) };
    let class_set = ClassSet::BinaryOp(binary_op);
    let _result = class_set.span();
}


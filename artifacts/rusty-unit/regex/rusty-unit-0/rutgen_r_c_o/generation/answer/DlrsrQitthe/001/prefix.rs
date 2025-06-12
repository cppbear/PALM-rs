// Answer 0

#[test]
fn test_from_bracketed_item_negated() {
    let span = Span::new(0, 10);
    let literal = Literal::new('a');
    let class_item = ClassSetItem::Literal(literal);
    let class_set = ClassSet::Item(class_item);
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: class_set,
    };
    let result = ClassInduct::from_bracketed(&class_bracketed);
}

#[test]
fn test_from_bracketed_item_non_negated() {
    let span = Span::new(0, 10);
    let literal = Literal::new('b');
    let class_item = ClassSetItem::Literal(literal);
    let class_set = ClassSet::Item(class_item);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: class_set,
    };
    let result = ClassInduct::from_bracketed(&class_bracketed);
}

#[test]
fn test_from_bracketed_binary_op_negated() {
    let span = Span::new(0, 10);
    let lhs_literal = Literal::new('c');
    let rhs_literal = Literal::new('d');
    let lhs = ClassSetItem::Literal(lhs_literal);
    let rhs = ClassSetItem::Literal(rhs_literal);
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(lhs)),
        rhs: Box::new(ClassSet::Item(rhs)),
    };
    let class_set = ClassSet::BinaryOp(binary_op);
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: class_set,
    };
    let result = ClassInduct::from_bracketed(&class_bracketed);
}

#[test]
fn test_from_bracketed_binary_op_non_negated() {
    let span = Span::new(0, 10);
    let lhs_literal = Literal::new('e');
    let rhs_literal = Literal::new('f');
    let lhs = ClassSetItem::Literal(lhs_literal);
    let rhs = ClassSetItem::Literal(rhs_literal);
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::Item(lhs)),
        rhs: Box::new(ClassSet::Item(rhs)),
    };
    let class_set = ClassSet::BinaryOp(binary_op);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: class_set,
    };
    let result = ClassInduct::from_bracketed(&class_bracketed);
}


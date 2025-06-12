// Answer 0

#[test]
fn test_class_set_span_item_literal() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Literal {
        span: Span,
    }
    
    let literal_span = Span { start: Position(0), end: Position(5) };
    let literal = Literal { span: literal_span };
    let class_set_item = ClassSetItem::Literal(literal);
    let class_set = ClassSet::Item(class_set_item);
    
    assert_eq!(class_set.span(), &literal_span);
}

#[test]
fn test_class_set_span_binary_op() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Position(usize);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct ClassSetBinaryOp {
        span: Span,
    }

    let binary_op_span = Span { start: Position(1), end: Position(3) };
    let binary_op = ClassSetBinaryOp { span: binary_op_span };
    let class_set_binary_op = ClassSet::BinaryOp(binary_op);
    
    assert_eq!(class_set_binary_op.span(), &binary_op_span);
}


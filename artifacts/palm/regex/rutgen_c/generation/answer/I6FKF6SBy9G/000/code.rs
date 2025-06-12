// Answer 0

#[test]
fn test_class_frame_union_debug() {
    struct Span;
    let item = ast::ClassSetItem::Empty(Span);
    let union_frame = ClassFrame::Union { head: &item, tail: &[] };
    let mut output = vec![];
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        union_frame.fmt(&mut formatter).unwrap();
    }
    assert_eq!(String::from_utf8(output).unwrap(), "Union");
}

#[test]
fn test_class_frame_binary_debug() {
    struct Span;
    let binary_op = ClassSetBinaryOp {
        span: Span,
        kind: ClassSetBinaryOpKind::And,
        lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('a')))),
        rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('b')))),
    };
    let binary_frame = ClassFrame::Binary { op: &binary_op };
    let mut output = vec![];
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        binary_frame.fmt(&mut formatter).unwrap();
    }
    assert_eq!(String::from_utf8(output).unwrap(), "Binary");
}

#[test]
fn test_class_frame_binary_lhs_debug() {
    struct Span;
    let lhs = ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('a')));
    let rhs = ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('b')));
    let binary_op = ClassSetBinaryOp {
        span: Span,
        kind: ClassSetBinaryOpKind::And,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };
    let binary_lhs_frame = ClassFrame::BinaryLHS {
        op: &binary_op,
        lhs: &lhs,
        rhs: &rhs,
    };
    let mut output = vec![];
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        binary_lhs_frame.fmt(&mut formatter).unwrap();
    }
    assert_eq!(String::from_utf8(output).unwrap(), "BinaryLHS");
}

#[test]
fn test_class_frame_binary_rhs_debug() {
    struct Span;
    let lhs = ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('a')));
    let rhs = ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('b')));
    let binary_op = ClassSetBinaryOp {
        span: Span,
        kind: ClassSetBinaryOpKind::And,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };
    let binary_rhs_frame = ClassFrame::BinaryRHS {
        op: &binary_op,
        rhs: &rhs,
    };
    let mut output = vec![];
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        binary_rhs_frame.fmt(&mut formatter).unwrap();
    }
    assert_eq!(String::from_utf8(output).unwrap(), "BinaryRHS");
}


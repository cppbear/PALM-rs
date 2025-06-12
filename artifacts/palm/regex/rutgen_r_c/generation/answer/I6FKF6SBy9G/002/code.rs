// Answer 0

#[test]
fn test_class_frame_debug_union() {
    let item = ast::ClassSetItem::Empty(Span::default());
    let lhs = Box::new(ClassSet::Item(item.clone()));
    let rhs = Box::new(ClassSet::Item(item));
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::SomeKind,
        lhs,
        rhs,
    };
    let class_frame = ClassFrame::BinaryLHS {
        op: &op,
        lhs: &class_frame,
        rhs: &class_frame,
    };
    let mut buffer = Vec::new();
    let result = class_frame.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "BinaryLHS");
}

#[test]
fn test_class_frame_debug_binary() {
    let item = ast::ClassSetItem::Empty(Span::default());
    let lhs = Box::new(ClassSet::Item(item.clone()));
    let rhs = Box::new(ClassSet::Item(item));
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::SomeKind,
        lhs,
        rhs,
    };
    let class_frame = ClassFrame::Binary {
        op: &op,
    };
    let mut buffer = Vec::new();
    let result = class_frame.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Binary");
}

#[test]
fn test_class_frame_debug_binary_rhs() {
    let item = ast::ClassSetItem::Empty(Span::default());
    let lhs = Box::new(ClassSet::Item(item.clone()));
    let rhs = Box::new(ClassSet::Item(item));
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::SomeKind,
        lhs,
        rhs,
    };
    let class_frame = ClassFrame::BinaryRHS {
        op: &op,
        rhs: &class_frame,
    };
    let mut buffer = Vec::new();
    let result = class_frame.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "BinaryRHS");
}


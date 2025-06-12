// Answer 0

#[test]
fn test_class_frame_union_debug() {
    use ast::{ClassSetItem, ClassSet};
    use std::fmt;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span;

    let span = Span;
    let head = ClassSetItem::Empty(span.clone());
    let tail: &[ClassSetItem] = &[];
    
    let frame = ClassFrame::Union {
        head: &head,
        tail: tail,
    };

    let result = format!("{:?}", frame);
    assert_eq!(result, "Union");
}

#[test]
fn test_class_frame_binary_debug() {
    use ast::{ClassSetItem, ClassSet};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span;

    let span = Span;
    
    let lhs = ClassSet::Item(ClassSetItem::Literal(Literal));
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal));
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let frame = ClassFrame::Binary {
        op: &binary_op,
    };

    let result = format!("{:?}", frame);
    assert_eq!(result, "Binary");
}

#[test]
fn test_class_frame_binary_lhs_debug() {
    use ast::{ClassSetItem, ClassSet};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span;

    let span = Span;

    let lhs = ClassSet::Item(ClassSetItem::Literal(Literal));
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal));
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let frame = ClassFrame::BinaryLHS {
        op: &binary_op,
        lhs: &lhs,
        rhs: &rhs,
    };

    let result = format!("{:?}", frame);
    assert_eq!(result, "BinaryLHS");
}

#[test]
fn test_class_frame_binary_rhs_debug() {
    use ast::{ClassSetItem, ClassSet};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span;

    let span = Span;

    let lhs = ClassSet::Item(ClassSetItem::Literal(Literal));
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal));
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let frame = ClassFrame::BinaryRHS {
        op: &binary_op,
        rhs: &rhs,
    };

    let result = format!("{:?}", frame);
    assert_eq!(result, "BinaryRHS");
}


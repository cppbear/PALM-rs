// Answer 0

#[test]
fn test_class_frame_binary_rhs() {
    struct Span;
    let span = Span;

    struct ClassSet;
    let lhs = ClassSet;
    let rhs = ClassSet;

    let binary_rhs_frame = ClassFrame::BinaryRHS { op: &ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Intersection, lhs: Box::new(lhs), rhs: Box::new(rhs) } };

    let mut formatter = std::fmt::Formatter::new();
    binary_rhs_frame.fmt(&mut formatter);
}

#[test]
fn test_class_frame_binary_rhs_with_empty_class() {
    struct Span;
    let span = Span;

    struct ClassSet;
    let lhs = ClassSet;
    let rhs = ClassSet;

    let binary_rhs_frame = ClassFrame::BinaryRHS { op: &ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs: Box::new(lhs), rhs: Box::new(rhs) } };

    let mut formatter = std::fmt::Formatter::new();
    binary_rhs_frame.fmt(&mut formatter);
}

#[test]
fn test_class_frame_binary_rhs_with_largest_values() {
    struct Span;
    let span = Span;

    struct ClassSet;
    let lhs = ClassSet;
    let rhs = ClassSet;

    let binary_rhs_frame = ClassFrame::BinaryRHS { op: &ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Intersection, lhs: Box::new(lhs), rhs: Box::new(rhs) } };

    let mut formatter = std::fmt::Formatter::new();
    binary_rhs_frame.fmt(&mut formatter);
}


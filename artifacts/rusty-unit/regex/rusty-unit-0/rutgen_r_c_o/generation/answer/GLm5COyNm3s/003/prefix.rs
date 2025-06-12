// Answer 0

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Parser;

#[test]
fn test_span_for_dot_with_zero_range() {
    let span = Span { start: Position(0), end: Position(0) };
    let primitive = Primitive::Dot(span);
    let _ = primitive.span();
}

#[test]
fn test_span_for_dot_with_single_range() {
    let span = Span { start: Position(1), end: Position(1) };
    let primitive = Primitive::Dot(span);
    let _ = primitive.span();
}

#[test]
fn test_span_for_dot_with_full_byte_range() {
    let span = Span { start: Position(255), end: Position(255) };
    let primitive = Primitive::Dot(span);
    let _ = primitive.span();
}

#[test]
fn test_span_for_dot_with_range_one_to255() {
    let span = Span { start: Position(1), end: Position(255) };
    let primitive = Primitive::Dot(span);
    let _ = primitive.span();
}

#[test]
fn test_span_for_dot_with_full_range() {
    let span = Span { start: Position(0), end: Position(255) };
    let primitive = Primitive::Dot(span);
    let _ = primitive.span();
}


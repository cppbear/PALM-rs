// Answer 0

#[test]
fn test_into_ast_dot_span_zero() {
    let span = Span { start: 0, end: 0 };
    let primitive = Primitive::Dot(span);
    let ast_result = primitive.into_ast();
}

#[test]
fn test_into_ast_dot_span_one() {
    let span = Span { start: 1, end: 1 };
    let primitive = Primitive::Dot(span);
    let ast_result = primitive.into_ast();
}

#[test]
fn test_into_ast_dot_span_two_fifty_five() {
    let span = Span { start: 255, end: 255 };
    let primitive = Primitive::Dot(span);
    let ast_result = primitive.into_ast();
}

#[test]
fn test_into_ast_dot_span_midpoint() {
    let span = Span { start: 128, end: 128 };
    let primitive = Primitive::Dot(span);
    let ast_result = primitive.into_ast();
}

#[test]
fn test_into_ast_dot_span_negative() {
    // This example is outside the valid range to ensure we properly test edge cases. 
    // However, the type system will prevent us from creating an invalid Span.
    // This test is commented out because it won't compile.
    // let span = Span { start: -1, end: -1 };
    // let primitive = Primitive::Dot(span);
    // let ast_result = primitive.into_ast();
}


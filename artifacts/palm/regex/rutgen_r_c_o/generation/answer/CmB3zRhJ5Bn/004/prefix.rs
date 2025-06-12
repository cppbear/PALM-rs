// Answer 0

#[test]
fn test_into_ast_assertion_word_boundary() {
    let assert = ast::Assertion {
        span: Span { start: 0, end: 10 },
        kind: AssertionKind::WordBoundary,
    };
    let primitive = Primitive::Assertion(assert);
    primitive.into_ast();
}

#[test]
fn test_into_ast_assertion_start() {
    let assert = ast::Assertion {
        span: Span { start: 0, end: 0 },
        kind: AssertionKind::Start,
    };
    let primitive = Primitive::Assertion(assert);
    primitive.into_ast();
}

#[test]
fn test_into_ast_assertion_end() {
    let assert = ast::Assertion {
        span: Span { start: 0, end: 5 },
        kind: AssertionKind::End,
    };
    let primitive = Primitive::Assertion(assert);
    primitive.into_ast();
}


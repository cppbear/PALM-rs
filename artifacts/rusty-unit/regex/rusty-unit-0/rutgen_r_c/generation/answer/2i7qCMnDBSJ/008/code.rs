// Answer 0

#[test]
fn test_ast_literal_has_subexprs() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Ast::Literal(Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' });
    assert_eq!(literal.has_subexprs(), false);
}

#[test]
fn test_ast_empty_has_subexprs() {
    let span = Span { start: Position(0), end: Position(0) };
    let empty = Ast::Empty(span);
    assert_eq!(empty.has_subexprs(), false);
}

#[test]
fn test_ast_flags_has_subexprs() {
    let span = Span { start: Position(0), end: Position(2) };
    let flags = Ast::Flags(SetFlags { span, flags: Flags::new() });
    assert_eq!(flags.has_subexprs(), false);
}

#[test]
fn test_ast_assertion_has_subexprs() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Ast::Assertion(Assertion { span, kind: AssertionKind::Start });
    assert_eq!(assertion.has_subexprs(), false);
}

#[test]
fn test_ast_dot_has_subexprs() {
    let span = Span { start: Position(0), end: Position(1) };
    let dot = Ast::Dot(span);
    assert_eq!(dot.has_subexprs(), false);
}


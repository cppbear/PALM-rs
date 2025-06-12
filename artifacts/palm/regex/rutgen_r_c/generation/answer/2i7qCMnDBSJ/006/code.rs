// Answer 0

#[test]
fn test_has_subexprs_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::Empty(span);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' };
    let ast = Ast::Literal(literal);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_flags() {
    let span = Span { start: Position(0), end: Position(4) };
    let flags = SetFlags { span, flags: Flags::empty() };
    let ast = Ast::Flags(flags);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Dot(span);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span, kind: AssertionKind::Start };
    let ast = Ast::Assertion(assertion);
    assert_eq!(ast.has_subexprs(), false);
}


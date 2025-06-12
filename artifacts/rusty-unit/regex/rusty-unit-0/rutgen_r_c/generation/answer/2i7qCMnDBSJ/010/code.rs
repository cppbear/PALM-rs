// Answer 0

#[test]
fn test_has_subexprs_empty() {
    let ast = Ast::Empty(Span { start: Position(0), end: Position(0) });
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_literal() {
    let ast = Ast::Literal(Literal { span: Span { start: Position(0), end: Position(1) }, kind: LiteralKind::Unicode('a'), c: 'a' });
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_flags() {
    let ast = Ast::Flags(SetFlags { span: Span { start: Position(0), end: Position(1) }, flags: Flags::new() });
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_assertion() {
    let ast = Ast::Assertion(Assertion { span: Span { start: Position(0), end: Position(1) }, kind: AssertionKind::Start });
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_dot() {
    let ast = Ast::Dot(Span { start: Position(0), end: Position(1) });
    assert_eq!(ast.has_subexprs(), false);
}


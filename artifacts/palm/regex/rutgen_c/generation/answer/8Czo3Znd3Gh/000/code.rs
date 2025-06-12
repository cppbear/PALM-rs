// Answer 0

#[test]
fn test_is_empty_with_empty_ast() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::Empty(span);
    assert!(ast.is_empty());
}

#[test]
fn test_is_empty_with_literal_ast() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal::Unicode('a');
    let ast = Ast::Literal(Literal { span, kind: literal, c: 'a' });
    assert!(!ast.is_empty());
}

#[test]
fn test_is_empty_with_assertion_ast() {
    let span = Span { start: Position(0), end: Position(2) };
    let assertion = Assertion { span, kind: AssertionKind::Start };
    let ast = Ast::Assertion(assertion);
    assert!(!ast.is_empty());
}

#[test]
fn test_is_empty_with_concat_ast() {
    let span = Span { start: Position(0), end: Position(3) };
    let concat = Concat { span, asts: vec![] };
    let ast = Ast::Concat(concat);
    assert!(!ast.is_empty());
}

#[test]
fn test_is_empty_with_group_ast() {
    let span = Span { start: Position(0), end: Position(4) };
    let group = Group { span, kind: GroupKind::Capturing(0), ast: Box::new(Ast::Empty(span)) };
    let ast = Ast::Group(group);
    assert!(!ast.is_empty());
}


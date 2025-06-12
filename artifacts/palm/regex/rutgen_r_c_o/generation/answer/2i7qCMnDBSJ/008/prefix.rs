// Answer 0

#[test]
fn test_has_no_subexprs_literal_unicode() {
    let span = Span { start: Position::default(), end: Position::default() };
    let literal = Ast::Literal(Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' });
    literal.has_subexprs();
}

#[test]
fn test_has_no_subexprs_literal_byte() {
    let span = Span { start: Position::default(), end: Position::default() };
    let literal = Ast::Literal(Literal { span, kind: LiteralKind::Byte(97), c: 'a' });
    literal.has_subexprs();
}

#[test]
fn test_has_no_subexprs_empty() {
    let span = Span { start: Position::default(), end: Position::default() };
    let empty = Ast::Empty(span);
    empty.has_subexprs();
}

#[test]
fn test_has_no_subexprs_flags() {
    let span = Span { start: Position::default(), end: Position::default() };
    let flags = SetFlags { span, flags: Flags::default() };
    let flags_ast = Ast::Flags(flags);
    flags_ast.has_subexprs();
}

#[test]
fn test_has_no_subexprs_assertion() {
    let span = Span { start: Position::default(), end: Position::default() };
    let assertion = Assertion { span, kind: AssertionKind::WordBoundary };
    let assertion_ast = Ast::Assertion(assertion);
    assertion_ast.has_subexprs();
}

#[test]
fn test_has_no_subexprs_dot() {
    let span = Span { start: Position::default(), end: Position::default() };
    let dot = Ast::Dot(span);
    dot.has_subexprs();
}


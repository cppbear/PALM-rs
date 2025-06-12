// Answer 0

#[test]
fn test_ast_has_subexprs_dot() {
    let span = Span { start: Position { index: 0 }, end: Position { index: 1 } };
    let ast = Ast::Dot(span);
    ast.has_subexprs();
}

#[test]
fn test_ast_has_subexprs_literal() {
    let span = Span { start: Position { index: 0 }, end: Position { index: 1 } };
    let literal = Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' };
    let ast = Ast::Literal(literal);
    ast.has_subexprs();
}

#[test]
fn test_ast_has_subexprs_empty() {
    let span = Span { start: Position { index: 0 }, end: Position { index: 0 } };
    let ast = Ast::Empty(span);
    ast.has_subexprs();
}

#[test]
fn test_ast_has_subexprs_flags() {
    let span = Span { start: Position { index: 0 }, end: Position { index: 3 } };
    let flags = SetFlags { span, flags: Flags::new() };
    let ast = Ast::Flags(flags);
    ast.has_subexprs();
}

#[test]
fn test_ast_has_subexprs_assertion() {
    let span = Span { start: Position { index: 0 }, end: Position { index: 1 } };
    let assertion = Assertion { span, kind: AssertionKind::Start };
    let ast = Ast::Assertion(assertion);
    ast.has_subexprs();
}


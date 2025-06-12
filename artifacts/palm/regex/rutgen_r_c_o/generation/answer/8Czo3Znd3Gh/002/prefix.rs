// Answer 0

#[test]
fn test_is_empty_with_empty_ast() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::Empty(span);
    ast.is_empty();
}

#[test]
fn test_is_empty_with_empty_ast_edge_case() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::Empty(span);
    ast.is_empty();
}


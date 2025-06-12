// Answer 0

#[test]
fn test_into_ast_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let concat = Concat { span, asts: Vec::new() };
    let result = concat.into_ast();
    assert_eq!(result, Ast::Empty(span));
}

#[test]
fn test_into_ast_single() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Literal(Literal); // Assuming Literal is a defined struct.
    let concat = Concat { span, asts: vec![ast.clone()] };
    let result = concat.into_ast();
    assert_eq!(result, ast);
}

#[test]
fn test_into_ast_multiple() {
    let span = Span { start: Position(0), end: Position(2) };
    let ast1 = Ast::Literal(Literal); // Assuming Literal is a defined struct.
    let ast2 = Ast::Dot(span); // Using Dot with the span as a sample AST.
    let concat = Concat { span, asts: vec![ast1.clone(), ast2.clone()] };
    let result = concat.into_ast();
    if let Ast::Concat(ref inner) = result {
        assert_eq!(inner.span, span);
        assert_eq!(inner.asts.len(), 2);
        assert_eq!(inner.asts[0], ast1);
        assert_eq!(inner.asts[1], ast2);
    } else {
        panic!("Expected result to be Ast::Concat.");
    }
}


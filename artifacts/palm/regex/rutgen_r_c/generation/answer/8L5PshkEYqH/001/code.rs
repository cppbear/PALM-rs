// Answer 0

#[test]
fn test_into_ast_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let alternation = Alternation { span, asts: Vec::new() };
    let result = alternation.into_ast();
    match result {
        Ast::Empty(_) => (),
        _ => panic!("Expected Ast::Empty for zero ASTs"),
    }
}

#[test]
fn test_into_ast_single() {
    let span = Span { start: Position(0), end: Position(1) };
    let single_ast = Ast::Literal(Literal::new('a')); // Assume Literal::new exists
    let alternation = Alternation { span, asts: vec![single_ast.clone()] };
    let result = alternation.into_ast();
    assert_eq!(result, single_ast, "Expected single AST to be returned");
}

#[test]
fn test_into_ast_multiple() {
    let span = Span { start: Position(0), end: Position(2) };
    let first_ast = Ast::Literal(Literal::new('a')); // Assume Literal::new exists
    let second_ast = Ast::Literal(Literal::new('b')); // Assume Literal::new exists
    let alternation = Alternation { span, asts: vec![first_ast.clone(), second_ast.clone()] };
    let result = alternation.into_ast();
    if let Ast::Alternation(_) = result {
        // Test passes
    } else {
        panic!("Expected Ast::Alternation for multiple ASTs");
    }
}


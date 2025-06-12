// Answer 0

#[test]
fn test_into_ast_zero_asts() {
    let span = Span { start: Position(0), end: Position(1) };
    let alternation = Alternation { span, asts: Vec::new() };
    let _ = alternation.into_ast();
}

#[test]
fn test_into_ast_one_ast() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Literal(Literal::new('a', span));
    let alternation = Alternation { span, asts: vec![ast] };
    let _ = alternation.into_ast();
}

#[test]
fn test_into_ast_multiple_asts() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast_1 = Ast::Literal(Literal::new('a', span));
    let ast_2 = Ast::Literal(Literal::new('b', span));
    let alternation = Alternation { span, asts: vec![ast_1, ast_2] };
    let _ = alternation.into_ast();
}


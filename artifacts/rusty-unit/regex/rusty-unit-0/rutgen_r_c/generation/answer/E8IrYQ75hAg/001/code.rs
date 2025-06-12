// Answer 0

#[test]
fn test_into_ast_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let concat = Concat { span, asts: Vec::new() };
    let result = concat.into_ast();
    match result {
        Ast::Empty(s) => assert_eq!(s, span),
        _ => panic!("Expected Ast::Empty but got something else."),
    }
}

#[test]
fn test_into_ast_single() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast_single = Ast::Literal(Literal(Some("\\".to_string()), span));
    let concat = Concat { span, asts: vec![ast_single] };
    let result = concat.into_ast();
    match result {
        Ast::Literal(_) => (),
        _ => panic!("Expected a single Ast::Literal but got something else."),
    }
}

#[test]
fn test_into_ast_multiple() {
    let span = Span { start: Position(0), end: Position(2) };
    let ast1 = Ast::Literal(Literal(Some("a".to_string()), span));
    let ast2 = Ast::Literal(Literal(Some("b".to_string()), span));
    let concat = Concat { span, asts: vec![ast1.clone(), ast2.clone()] };
    let result = concat.into_ast();
    match result {
        Ast::Concat(ref c) => {
            assert_eq!(c.span, span);
            assert_eq!(c.asts.len(), 2);
        },
        _ => panic!("Expected Ast::Concat but got something else."),
    }
}


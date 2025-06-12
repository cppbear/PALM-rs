// Answer 0

#[test]
fn test_into_ast_with_no_asts() {
    let span = Span { start: Position(0), end: Position(0) };
    let concat = Concat { span, asts: vec![] };
    let result = concat.into_ast();
    match result {
        Ast::Empty(_) => (),
        _ => panic!("Expected Ast::Empty"),
    }
}

#[test]
fn test_into_ast_with_one_ast() {
    let span = Span { start: Position(0), end: Position(5) };
    let ast = Ast::Literal(Literal { value: 'a' }); // Assuming Literal has a suitable construction
    let concat = Concat { span, asts: vec![ast.clone()] };
    let result = concat.into_ast();
    assert_eq!(result, ast);
}

#[test]
fn test_into_ast_with_multiple_asts() {
    let span = Span { start: Position(0), end: Position(10) };
    let ast1 = Ast::Literal(Literal { value: 'a' });
    let ast2 = Ast::Literal(Literal { value: 'b' });
    let concat = Concat { span, asts: vec![ast1.clone(), ast2.clone()] };
    let result = concat.into_ast();
    match result {
        Ast::Concat(ref c) => {
            assert_eq!(c.asts.len(), 2);
            assert_eq!(c.asts[0], ast1);
            assert_eq!(c.asts[1], ast2);
        }
        _ => panic!("Expected Ast::Concat"),
    }
}


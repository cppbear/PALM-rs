// Answer 0

#[derive(Debug)]
struct Position(usize);

#[test]
fn test_into_ast_empty() {
    let valid_span = Span { start: Position(0), end: Position(0) };
    let concat = Concat { span: valid_span, asts: Vec::new() };
    let ast = concat.into_ast();
}

#[test]
fn test_into_ast_empty_with_non_zero_length() {
    let valid_span = Span { start: Position(1), end: Position(2) };
    let concat = Concat { span: valid_span, asts: vec![Ast::Literal(Literal::from('a'))] };
    let ast = concat.into_ast();
}

#[test]
fn test_into_ast_empty_with_multiple_elements() {
    let valid_span = Span { start: Position(2), end: Position(3) };
    let concat = Concat { span: valid_span, asts: vec![Ast::Literal(Literal::from('a')), Ast::Literal(Literal::from('b'))] };
    let ast = concat.into_ast();
}


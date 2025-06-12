// Answer 0

#[test]
fn test_ast_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::Empty(span);
    let mut output = String::new();
    let result = write!(&mut output, "{}", ast);
    
    assert!(result.is_ok());
}

#[test]
fn test_ast_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal::Unicode('a');
    let ast = Ast::Literal(literal);
    let mut output = String::new();
    let result = write!(&mut output, "{}", ast);
    
    assert!(result.is_ok());
}

#[test]
fn test_ast_concatenation() {
    let span1 = Span { start: Position(0), end: Position(1) };
    let literal1 = Literal::Unicode('a');
    let ast1 = Ast::Literal(literal1);

    let span2 = Span { start: Position(1), end: Position(2) };
    let literal2 = Literal::Unicode('b');
    let ast2 = Ast::Literal(literal2);

    let concat_ast = Ast::Concat(Concat { span: Span { start: Position(0), end: Position(2) }, asts: vec![ast1, ast2] });
    let mut output = String::new();
    let result = write!(&mut output, "{}", concat_ast);
    
    assert!(result.is_ok());
}

#[test]
fn test_ast_group() {
    let span = Span { start: Position(0), end: Position(2) };
    let literal = Literal::Unicode('a');
    let ast = Ast::Group(Group { span, kind: GroupKind::Capturing(0), ast: Box::new(Ast::Literal(literal)) });
    let mut output = String::new();
    let result = write!(&mut output, "{}", ast);
    
    assert!(result.is_ok());
}

#[test]
fn test_ast_alternation() {
    let span1 = Span { start: Position(0), end: Position(1) };
    let literal1 = Literal::Unicode('a');
    let ast1 = Ast::Literal(literal1);

    let span2 = Span { start: Position(1), end: Position(2) };
    let literal2 = Literal::Unicode('b');
    let ast2 = Ast::Literal(literal2);

    let alternation_ast = Ast::Alternation(Alternation { span: Span { start: Position(0), end: Position(2) }, asts: vec![ast1, ast2] });
    let mut output = String::new();
    let result = write!(&mut output, "{}", alternation_ast);
    
    assert!(result.is_ok());
}


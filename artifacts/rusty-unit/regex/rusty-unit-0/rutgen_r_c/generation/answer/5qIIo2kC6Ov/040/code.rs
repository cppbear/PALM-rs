// Answer 0

#[test]
fn test_parse_with_comments_empty() {
    let mut parser = Parser::new();
    let result = parser.parse_with_comments("");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(with_comments.ast, Ast::Concat { span: Span { start: 0, end: 0 }, asts: vec![] });
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_single_literal() {
    let mut parser = Parser::new();
    let result = parser.parse_with_comments("a");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(with_comments.ast, Ast::Concat { span: Span { start: 0, end: 1 }, asts: vec![Ast::Literal(ast::Literal { span: Span { start: 0, end: 1 }, kind: ast::LiteralKind::Verbatim, c: 'a' })] });
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_capture_group() {
    let mut parser = Parser::new();
    let result = parser.parse_with_comments("(abc)");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(with_comments.ast, Ast::Group(ast::Group { span: Span { start: 0, end: 5 }, ast: Box::new(Ast::Concat { span: Span { start: 0, end: 5 }, asts: vec![Ast::Literal(ast::Literal { span: Span { start: 1, end: 2 }, kind: ast::LiteralKind::Verbatim, c: 'a' }), Ast::Literal(ast::Literal { span: Span { start: 2, end: 3 }, kind: ast::LiteralKind::Verbatim, c: 'b' }), Ast::Literal(ast::Literal { span: Span { start: 3, end: 4 }, kind: ast::LiteralKind::Verbatim, c: 'c' })] }) }));
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_multiple_literals_with_comments() {
    let mut parser = Parser::new();
    let result = parser.parse_with_comments("a # comment\nb");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(with_comments.ast, Ast::Concat { span: Span { start: 0, end: 10 }, asts: vec![Ast::Literal(ast::Literal { span: Span { start: 0, end: 1 }, kind: ast::LiteralKind::Verbatim, c: 'a' }), Ast::Literal(ast::Literal { span: Span { start: 6, end: 7 }, kind: ast::LiteralKind::Verbatim, c: 'b' })] });
    assert_eq!(with_comments.comments.len(), 1);
    assert_eq!(with_comments.comments[0].span, Span { start: 1, end: 10 });
    assert_eq!(with_comments.comments[0].comment, " comment");
}

#[test]
fn test_parse_with_comments_nested_groups() {
    let mut parser = Parser::new();
    let result = parser.parse_with_comments("(a(bc)d)");
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(with_comments.ast.is_kind_of::<Ast::Group>());
}


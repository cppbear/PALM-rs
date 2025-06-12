// Answer 0

#[test]
fn test_is_empty_literal_unicode() {
    let ast = Ast::Literal(Literal::Unicode('a'));
    let result = ast.is_empty();
}

#[test]
fn test_is_empty_literal_byte() {
    let ast = Ast::Literal(Literal::Byte(0));
    let result = ast.is_empty();
}

#[test]
fn test_is_empty_group() {
    let ast = Ast::Group(Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::Capture(1),
        ast: Box::new(Ast::Literal(Literal::Unicode('b')))
    });
    let result = ast.is_empty();
}


// Answer 0

#[test]
fn test_has_subexprs_alternation() {
    let span = Span { start: Position(0), end: Position(10) };
    let ast1 = Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Char('a'), c: 'a' });
    let ast2 = Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Char('b'), c: 'b' });
    let alternation = Ast::Alternation(Alternation {
        span,
        asts: vec![ast1, ast2],
    });

    assert!(alternation.has_subexprs());
}

#[test]
fn test_has_subexprs_empty_alternation() {
    let span = Span { start: Position(0), end: Position(5) };
    let alternation = Ast::Alternation(Alternation {
        span,
        asts: vec![],
    });

    assert!(alternation.has_subexprs());
}

#[test]
fn test_has_subexprs_group() {
    let span = Span { start: Position(5), end: Position(15) };
    let group = Ast::Group(Group {
        span: span.clone(),
        kind: GroupKind::Capturing(1),
        hir: Box::new(Ast::Literal(Literal { span, kind: LiteralKind::Char('c'), c: 'c' })),
    });

    assert!(group.has_subexprs());
}


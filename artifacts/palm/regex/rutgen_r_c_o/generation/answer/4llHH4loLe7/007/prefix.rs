// Answer 0

#[test]
fn test_visit_post_with_alternation_valid_expressions() {
    let literal_a = ast::Ast::Literal(ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Char('a'),
        c: 'a',
    });
    let literal_b = ast::Ast::Literal(ast::Literal {
        span: Span { start: 1, end: 2 },
        kind: ast::LiteralKind::Char('b'),
        c: 'b',
    });
    let alternation = ast::Ast::Alternation(vec![literal_a, literal_b]);
    
    let translator = Translator {
        stack: RefCell::new(vec![
            HirFrame::Expr(Hir::literal(ast::Literal {
                span: Span { start: 2, end: 3 },
                kind: ast::LiteralKind::Char('c'),
                c: 'c',
            })),
            HirFrame::Expr(Hir::empty()),
        ]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let mut visitor = TranslatorI::new(&translator, "abcd");
    visitor.visit_post(&alternation).unwrap();
}

#[test]
fn test_visit_post_with_alternation_empty_expression() {
    let literal_a = ast::Ast::Literal(ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Char('a'),
        c: 'a',
    });
    let alternation = ast::Ast::Alternation(vec![literal_a]);

    let translator = Translator {
        stack: RefCell::new(vec![HirFrame::Expr(Hir::empty())]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let mut visitor = TranslatorI::new(&translator, "a");
    visitor.visit_post(&alternation).unwrap();
}

#[test]
#[should_panic]
fn test_visit_post_with_alternation_no_frames() {
    let literal_a = ast::Ast::Literal(ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Char('a'),
        c: 'a',
    });
    let alternation = ast::Ast::Alternation(vec![literal_a]);

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let mut visitor = TranslatorI::new(&translator, "a");
    visitor.visit_post(&alternation).unwrap();
}

#[test]
fn test_visit_post_with_alternation_multiple_expressions() {
    let literal_a = ast::Ast::Literal(ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Char('a'),
        c: 'a',
    });
    let literal_b = ast::Ast::Literal(ast::Literal {
        span: Span { start: 1, end: 2 },
        kind: ast::LiteralKind::Char('b'),
        c: 'b',
    });
    let alternation = ast::Ast::Alternation(vec![literal_a.clone(), literal_b.clone()]);

    let translator = Translator {
        stack: RefCell::new(vec![
            HirFrame::Expr(Hir::literal(ast::Literal {
                span: Span { start: 2, end: 3 },
                kind: ast::LiteralKind::Char('c'),
                c: 'c',
            })),
            HirFrame::Expr(Hir::literal(ast::Literal {
                span: Span { start: 3, end: 4 },
                kind: ast::LiteralKind::Char('d'),
                c: 'd',
            })),
        ]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let mut visitor = TranslatorI::new(&translator, "ab");
    visitor.visit_post(&alternation).unwrap();
}


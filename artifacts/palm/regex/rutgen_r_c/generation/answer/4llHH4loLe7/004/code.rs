// Answer 0

fn test_visit_post_empty_ast() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    
    let mut translator_instance = TranslatorI::new(&translator, "");

    let ast = Ast::Empty(Span { start: 0, end: 0 });
    let result = translator_instance.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(translator_instance.pop().unwrap().unwrap_expr().kind(), &HirKind::Empty);
}

fn test_visit_post_literal_ast() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let mut translator_instance = TranslatorI::new(&translator, "");
    
    let ast = Ast::Literal(Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Char,
        c: 'a',
    });

    let result = translator_instance.visit_post(&ast);
    assert!(result.is_ok());
    assert!(matches!(translator_instance.pop().unwrap(), HirFrame::Expr(_)));
}

fn test_visit_post_dot_ast() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let mut translator_instance = TranslatorI::new(&translator, "");

    let ast = Ast::Dot(Span { start: 0, end: 1 });
    let result = translator_instance.visit_post(&ast);
    assert!(result.is_ok());
    assert!(matches!(translator_instance.pop().unwrap(), HirFrame::Expr(_)));
}

fn test_visit_post_concat_ast() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let mut translator_instance = TranslatorI::new(&translator, "");

    let literal_expr_1 = Hir::literal(Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Char,
        c: 'a',
    });
    translator_instance.push(HirFrame::Expr(literal_expr_1));

    let literal_expr_2 = Hir::literal(Literal {
        span: Span { start: 1, end: 2 },
        kind: LiteralKind::Char,
        c: 'b',
    });
    translator_instance.push(HirFrame::Expr(literal_expr_2));
    
    let ast = Ast::Concat(vec![]);
    let result = translator_instance.visit_post(&ast);
    assert!(result.is_ok());

    let expr = translator_instance.pop().unwrap().unwrap_expr();
    assert_eq!(expr.kind(), &HirKind::Concat(vec![
        Hir::literal(Literal { 
            span: Span { start: 0, end: 1 }, 
            kind: LiteralKind::Char, 
            c: 'a' 
        }),
        Hir::literal(Literal { 
            span: Span { start: 1, end: 2 }, 
            kind: LiteralKind::Char, 
            c: 'b' 
        }),
    ]));
}

fn test_visit_post_alternation_ast() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let mut translator_instance = TranslatorI::new(&translator, "");

    let literal_expr_1 = Hir::literal(Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Char,
        c: 'a',
    });
    translator_instance.push(HirFrame::Expr(literal_expr_1));

    let literal_expr_2 = Hir::literal(Literal {
        span: Span { start: 1, end: 2 },
        kind: LiteralKind::Char,
        c: 'b',
    });
    translator_instance.push(HirFrame::Expr(literal_expr_2));

    let ast = Ast::Alternation(vec![]);
    let result = translator_instance.visit_post(&ast);
    assert!(result.is_ok());

    let expr = translator_instance.pop().unwrap().unwrap_expr();
    assert_eq!(expr.kind(), &HirKind::Alternation(vec![
        Hir::literal(Literal { 
            span: Span { start: 0, end: 1 }, 
            kind: LiteralKind::Char, 
            c: 'a' 
        }),
        Hir::literal(Literal { 
            span: Span { start: 1, end: 2 }, 
            kind: LiteralKind::Char, 
            c: 'b' 
        }),
    ]));
}


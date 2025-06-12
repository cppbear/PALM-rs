// Answer 0

fn test_visit_post_alternation_with_valid_expr() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "a|b";
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let lit_a = Ast::Literal(Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Character,
        c: 'a',
    });
    let lit_b = Ast::Literal(Literal {
        span: Span { start: 2, end: 3 },
        kind: LiteralKind::Character,
        c: 'b',
    });
    translator.push(HirFrame::Expr(Hir::literal(lit_a.clone())));
    translator.push(HirFrame::Expr(Hir::literal(lit_b.clone())));

    let alternation_ast = Ast::Alternation(vec![lit_a, lit_b]);
    let result = translator.visit_post(&alternation_ast);
    
    assert!(result.is_ok());
}

fn test_visit_post_alternation_with_empty_exprs() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "a|b";
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let alternation_ast = Ast::Alternation(vec![]);
    let result = translator.visit_post(&alternation_ast);
    
    assert!(result.is_ok());
}

fn test_visit_post_alternation_with_popped_empty_expr() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "a|b";
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let lit_a = Ast::Literal(Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Character,
        c: 'a',
    });
    translator.push(HirFrame::Expr(Hir::literal(lit_a.clone())));

    let alternation_ast = Ast::Alternation(vec![lit_a, lit_a.clone()]); // push same lit twice
    let result = translator.visit_post(&alternation_ast);
    
    assert!(result.is_ok());
}

fn test_visit_post_alternation_no_exprs() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "a|b";
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let alternation_ast = Ast::Alternation(vec![]);
    let result = translator.visit_post(&alternation_ast);
    
    assert_eq!(result.is_ok(), true);
}


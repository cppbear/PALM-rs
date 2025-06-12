// Answer 0

#[test]
fn test_hir_repetition_zero_or_one() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test");
    
    let repetition = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::new("test")),
    };
    let expr = Hir::empty();
    
    translator_i.hir_repetition(&repetition, expr);
}

#[test]
fn test_hir_repetition_at_least() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test");
    
    let repetition = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(5)) },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::new("test")),
    };
    let expr = Hir::empty();
    
    translator_i.hir_repetition(&repetition, expr);
}

#[test]
fn test_hir_repetition_exactly() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test");
    
    let repetition = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(3)) },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::new("test")),
    };
    let expr = Hir::empty();
    
    translator_i.hir_repetition(&repetition, expr);
}

#[test]
fn test_hir_repetition_bounded() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test");
    
    let repetition = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)) },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::new("test")),
    };
    let expr = Hir::empty();
    
    translator_i.hir_repetition(&repetition, expr);
}

#[test]
fn test_hir_repetition_zero_or_one_swap_greed() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: Some(true),
            unicode: None,
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test");
    
    let repetition = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::new("test")),
    };
    let expr = Hir::empty();
    
    translator_i.hir_repetition(&repetition, expr);
}


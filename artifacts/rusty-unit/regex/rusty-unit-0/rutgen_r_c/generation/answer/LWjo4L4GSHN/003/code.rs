// Answer 0

fn test_hir_repetition_zero_or_one() {
    struct MockAstRepetition {
        op: ast::RepetitionOp,
        greedy: bool,
    }
    
    impl MockAstRepetition {
        fn new(kind: ast::RepetitionKind, greedy: bool) -> Self {
            Self {
                op: ast::RepetitionOp { kind },
                greedy,
            }
        }
    }

    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { swap_greed: Some(true), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    
    let translator_i = TranslatorI::new(&translator, "test");
    
    let rep = MockAstRepetition::new(ast::RepetitionKind::ZeroOrOne, true);
    let expr = Hir::literal(hir::Literal::from_char('a'));
    let result = translator_i.hir_repetition(&rep, expr);
    assert_eq!(result.kind(), &HirKind::Repetition { kind: hir::RepetitionKind::ZeroOrOne, greedy: false });
}

fn test_hir_repetition_zero_or_more() {
    struct MockAstRepetition {
        op: ast::RepetitionOp,
        greedy: bool,
    }

    impl MockAstRepetition {
        fn new(kind: ast::RepetitionKind, greedy: bool) -> Self {
            Self {
                op: ast::RepetitionOp { kind },
                greedy,
            }
        }
    }

    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { swap_greed: Some(true), ..Default::default() }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "test");
    
    let rep = MockAstRepetition::new(ast::RepetitionKind::ZeroOrMore, true);
    let expr = Hir::literal(hir::Literal::from_char('b'));
    let result = translator_i.hir_repetition(&rep, expr);
    assert_eq!(result.kind(), &HirKind::Repetition { kind: hir::RepetitionKind::ZeroOrMore, greedy: false });
}

fn test_hir_repetition_at_least() {
    struct MockAstRepetition {
        op: ast::RepetitionOp,
        greedy: bool,
    }

    impl MockAstRepetition {
        fn new(kind: ast::RepetitionKind, greedy: bool) -> Self {
            Self {
                op: ast::RepetitionOp { kind },
                greedy,
            }
        }
    }

    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { swap_greed: Some(true), ..Default::default() }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "test");
    
    let rep = MockAstRepetition::new(ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(3)), true);
    let expr = Hir::literal(hir::Literal::from_char('c'));
    let result = translator_i.hir_repetition(&rep, expr);
    assert_eq!(result.kind(), &HirKind::Repetition { kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(3)), greedy: false });
}

fn test_hir_repetition_bounded() {
    struct MockAstRepetition {
        op: ast::RepetitionOp,
        greedy: bool,
    }

    impl MockAstRepetition {
        fn new(kind: ast::RepetitionKind, greedy: bool) -> Self {
            Self {
                op: ast::RepetitionOp { kind },
                greedy,
            }
        }
    }

    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { swap_greed: Some(true), ..Default::default() }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "test");
    
    let rep = MockAstRepetition::new(ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)), true);
    let expr = Hir::literal(hir::Literal::from_char('d'));
    let result = translator_i.hir_repetition(&rep, expr);
    assert_eq!(result.kind(), &HirKind::Repetition { kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)), greedy: false });
}


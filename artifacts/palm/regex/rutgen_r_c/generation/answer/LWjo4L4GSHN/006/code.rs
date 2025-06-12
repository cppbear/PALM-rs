// Answer 0

#[test]
fn test_hir_repetition_zero_or_one() {
    struct TestTranslator {
        flags: Flags,
    }

    impl TranslatorI<'_, '_> {
        fn new(trans: &TestTranslator, pattern: &str) -> Self {
            TranslatorI { trans, pattern }
        }

        fn flags(&self) -> Flags {
            self.trans.flags.clone()
        }
    }

    let flags = Flags { swap_greed: Some(false), ..Flags::default() };
    let translator = TestTranslator { flags };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne, ..Default::default() },
        greedy: true,
        span: Default::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let expr = Hir::empty();

    let result = translator_i.hir_repetition(&rep, expr);
    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition { 
        kind: hir::RepetitionKind::ZeroOrOne, 
        greedy: true, 
        hir: Box::new(Hir::empty()) 
    }));
}

#[test]
fn test_hir_repetition_at_least() {
    struct TestTranslator {
        flags: Flags,
    }

    impl TranslatorI<'_, '_> {
        fn new(trans: &TestTranslator, pattern: &str) -> Self {
            TranslatorI { trans, pattern }
        }

        fn flags(&self) -> Flags {
            self.trans.flags.clone()
        }
    }

    let flags = Flags { swap_greed: Some(false), ..Flags::default() };
    let translator = TestTranslator { flags };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(3)), ..Default::default() },
        greedy: true,
        span: Default::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let expr = Hir::empty();

    let result = translator_i.hir_repetition(&rep, expr);
    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition { 
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(3)), 
        greedy: true, 
        hir: Box::new(Hir::empty()) 
    }));
}

#[test]
fn test_hir_repetition_exactly() {
    struct TestTranslator {
        flags: Flags,
    }

    impl TranslatorI<'_, '_> {
        fn new(trans: &TestTranslator, pattern: &str) -> Self {
            TranslatorI { trans, pattern }
        }

        fn flags(&self) -> Flags {
            self.trans.flags.clone()
        }
    }

    let flags = Flags { swap_greed: Some(false), ..Flags::default() };
    let translator = TestTranslator { flags };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(2)), ..Default::default() },
        greedy: true,
        span: Default::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let expr = Hir::empty();

    let result = translator_i.hir_repetition(&rep, expr);
    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition { 
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(2)), 
        greedy: true, 
        hir: Box::new(Hir::empty()) 
    }));
}

#[test]
fn test_hir_repetition_bounded() {
    struct TestTranslator {
        flags: Flags,
    }

    impl TranslatorI<'_, '_> {
        fn new(trans: &TestTranslator, pattern: &str) -> Self {
            TranslatorI { trans, pattern }
        }

        fn flags(&self) -> Flags {
            self.trans.flags.clone()
        }
    }

    let flags = Flags { swap_greed: Some(false), ..Flags::default() };
    let translator = TestTranslator { flags };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)), ..Default::default() },
        greedy: true,
        span: Default::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let expr = Hir::empty();

    let result = translator_i.hir_repetition(&rep, expr);
    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition { 
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)), 
        greedy: true, 
        hir: Box::new(Hir::empty()) 
    }));
}


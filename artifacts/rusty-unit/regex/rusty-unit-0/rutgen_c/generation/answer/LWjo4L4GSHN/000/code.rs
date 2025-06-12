// Answer 0

#[test]
fn test_hir_repetition_zero_or_one() {
    struct MockTranslator;
    let translator = MockTranslator;
    let expr = Hir::empty(); // empty expression for testing

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: true,
        span: Span::default(),
        ast: Box::new(ast::Literal::default()), // Using a default literal for testing
    };

    let result = translator.hir_repetition(&rep, expr);

    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::empty()),
    }));
}

#[test]
fn test_hir_repetition_zero_or_more() {
    struct MockTranslator;
    let translator = MockTranslator;
    let expr = Hir::empty();

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let result = translator.hir_repetition(&rep, expr);

    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(Hir::empty()),
    }));
}

#[test]
fn test_hir_repetition_one_or_more() {
    struct MockTranslator;
    let translator = MockTranslator;
    let expr = Hir::empty();

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
        span: Span::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let result = translator.hir_repetition(&rep, expr);

    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir::empty()),
    }));
}

#[test]
fn test_hir_repetition_exactly() {
    struct MockTranslator;
    let translator = MockTranslator;
    let expr = Hir::empty();

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(3)),
        },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let result = translator.hir_repetition(&rep, expr);

    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(3)),
        greedy: false,
        hir: Box::new(Hir::empty()),
    }));
}

#[test]
fn test_hir_repetition_at_least() {
    struct MockTranslator;
    let translator = MockTranslator;
    let expr = Hir::empty();

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(2)),
        },
        greedy: true,
        span: Span::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let result = translator.hir_repetition(&rep, expr);

    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(2)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    }));
}

#[test]
fn test_hir_repetition_bounded() {
    struct MockTranslator;
    let translator = MockTranslator;
    let expr = Hir::empty();

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(1, 5)),
        },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::default()),
    };

    let result = translator.hir_repetition(&rep, expr);

    assert_eq!(result.kind(), &HirKind::Repetition(hir::Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 5)),
        greedy: false,
        hir: Box::new(Hir::empty()),
    }));
}


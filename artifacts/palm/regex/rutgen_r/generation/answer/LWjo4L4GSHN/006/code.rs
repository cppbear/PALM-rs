// Answer 0

#[test]
fn test_hir_repetition_zero_or_one() {
    struct TestFlags {
        swap_greed: bool,
    }

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            self.swap_greed
        }
    }

    struct TestRepetition {
        op: ast::RepetitionKind,
        greedy: bool,
    }

    let flags = TestFlags { swap_greed: false };
    let expr = Hir::some_expr(); // Assume this initializes a valid Hir expression

    let rep = TestRepetition {
        op: ast::RepetitionKind::ZeroOrOne,
        greedy: true,
    };

    let result = flags.hir_repetition(&rep, expr);
    assert_eq!(result.kind, hir::RepetitionKind::ZeroOrOne);
    assert_eq!(result.greedy, true);
}

#[test]
fn test_hir_repetition_exactly() {
    struct TestFlags {
        swap_greed: bool,
    }

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            self.swap_greed
        }
    }

    struct TestRepetition {
        op: ast::RepetitionKind,
        greedy: bool,
    }

    let flags = TestFlags { swap_greed: false };
    let expr = Hir::some_expr(); // Assume this initializes a valid Hir expression

    let rep = TestRepetition {
        op: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(3)),
        greedy: true,
    };

    let result = flags.hir_repetition(&rep, expr);
    assert_eq!(result.kind, hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(3)));
    assert_eq!(result.greedy, true);
}

#[test]
fn test_hir_repetition_at_least() {
    struct TestFlags {
        swap_greed: bool,
    }

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            self.swap_greed
        }
    }

    struct TestRepetition {
        op: ast::RepetitionKind,
        greedy: bool,
    }

    let flags = TestFlags { swap_greed: false };
    let expr = Hir::some_expr(); // Assume this initializes a valid Hir expression

    let rep = TestRepetition {
        op: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(2)),
        greedy: false,
    };

    let result = flags.hir_repetition(&rep, expr);
    assert_eq!(result.kind, hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(2)));
    assert_eq!(result.greedy, false);
}

#[test]
fn test_hir_repetition_bounded() {
    struct TestFlags {
        swap_greed: bool,
    }

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            self.swap_greed
        }
    }

    struct TestRepetition {
        op: ast::RepetitionKind,
        greedy: bool,
    }

    let flags = TestFlags { swap_greed: false };
    let expr = Hir::some_expr(); // Assume this initializes a valid Hir expression

    let rep = TestRepetition {
        op: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(1, 5)),
        greedy: true,
    };

    let result = flags.hir_repetition(&rep, expr);
    assert_eq!(result.kind, hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 5)));
    assert_eq!(result.greedy, true);
}


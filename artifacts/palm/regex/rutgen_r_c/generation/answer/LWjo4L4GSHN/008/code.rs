// Answer 0

#[test]
fn test_hir_repetition_one_or_more_non_greedy() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }

    impl TestTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags {
                    swap_greed: Some(false),
                    ..Default::default()
                }),
            }
        }

        fn flags(&self) -> Flags {
            self.flags.get()
        }
    }

    let trans = TestTranslator::new();

    // Create an AST Repetition with kind OneOrMore
    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
    };

    // Create a simple Hir expression
    let expr = Hir::empty();

    // Now test the hir_repetition
    let result = trans.hir_repetition(&rep, expr);

    // Check that the resulting Hir has the expected characteristics
    if let hir::HirKind::Repetition(ref repetition) = result.kind() {
        assert_eq!(repetition.kind, hir::RepetitionKind::OneOrMore);
        assert!(repetition.greedy); // should be true
    } else {
        panic!("Expected Hir kind to be Repetition");
    }
}

#[test]
fn test_hir_repetition_one_or_more_greedy() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }

    impl TestTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags {
                    swap_greed: Some(false),
                    ..Default::default()
                }),
            }
        }

        fn flags(&self) -> Flags {
            self.flags.get()
        }
    }

    let trans = TestTranslator::new();

    // Create an AST Repetition with kind OneOrMore
    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: false,
    };

    // Create a simple Hir expression
    let expr = Hir::empty();

    // Now test the hir_repetition
    let result = trans.hir_repetition(&rep, expr);

    // Check that the resulting Hir has the expected characteristics
    if let hir::HirKind::Repetition(ref repetition) = result.kind() {
        assert_eq!(repetition.kind, hir::RepetitionKind::OneOrMore);
        assert!(!repetition.greedy); // should be false based on input
    } else {
        panic!("Expected Hir kind to be Repetition");
    }
}


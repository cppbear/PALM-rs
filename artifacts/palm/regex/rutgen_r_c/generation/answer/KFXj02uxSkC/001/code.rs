// Answer 0

#[test]
fn test_compile_one_needs_dotstar_err() {
    use syntax::hir::{self, Hir};

    struct TestExpr {
        is_anchored_start: bool,
        is_anchored_end: bool,
    }

    impl TestExpr {
        fn is_anchored_start(&self) -> bool {
            self.is_anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.is_anchored_end
        }
    }

    let dummy_expr = TestExpr {
        is_anchored_start: false,
        is_anchored_end: false,
    };

    let result = Compiler::new()
        .dfa(true)
        .compile_one(&dummy_expr);

    assert!(result.is_err());
}

#[test]
fn test_compile_one_needs_dotstar_success() {
    use syntax::hir::{self, Hir};

    struct TestExpr {
        is_anchored_start: bool,
        is_anchored_end: bool,
    }

    impl TestExpr {
        fn is_anchored_start(&self) -> bool {
            self.is_anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.is_anchored_end
        }
    }

    let dummy_expr = TestExpr {
        is_anchored_start: false,
        is_anchored_end: false,
    };

    let result = Compiler::new()
        .dfa(true)
        .only_utf8(true)
        .compile_one(&dummy_expr);

    assert!(result.is_ok());
    let program = result.unwrap();
    assert!(program.needs_dotstar());
    assert_eq!(program.start, 0);
    assert_eq!(program.matches.len(), 1);
}


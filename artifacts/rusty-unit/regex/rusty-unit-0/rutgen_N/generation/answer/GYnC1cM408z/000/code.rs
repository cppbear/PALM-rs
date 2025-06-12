// Answer 0

#[test]
fn test_compile_many_with_anchored_start_end() {
    struct TestSelf {
        compiled: Compiled,
    }

    struct Compiled {
        is_anchored_start: bool,
        is_anchored_end: bool,
        start: usize,
        matches: Vec<usize>,
    }

    struct Hir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl Hir {
        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let exprs = vec![
        Hir { anchored_start: true, anchored_end: false },
        Hir { anchored_start: true, anchored_end: false },
    ];

    let mut test_instance = TestSelf {
        compiled: Compiled {
            is_anchored_start: false,
            is_anchored_end: false,
            start: 0,
            matches: Vec::new(),
        },
    };

    let result = compile_many(test_instance, &exprs);
    assert!(result.is_ok());
    assert!(test_instance.compiled.is_anchored_start);
    assert!(!test_instance.compiled.is_anchored_end);
}

#[test]
fn test_compile_many_with_no_exprs() {
    struct TestSelf {
        compiled: Compiled,
    }

    struct Compiled {
        is_anchored_start: bool,
        is_anchored_end: bool,
        start: usize,
        matches: Vec<usize>,
    }

    let exprs: Vec<Hir> = Vec::new();
    let mut test_instance = TestSelf {
        compiled: Compiled {
            is_anchored_start: false,
            is_anchored_end: false,
            start: 0,
            matches: Vec::new(),
        },
    };

    let result = compile_many(test_instance, &exprs);
    assert!(result.is_err());
}

#[test]
fn test_compile_many_with_single_expr() {
    struct TestSelf {
        compiled: Compiled,
    }

    struct Compiled {
        is_anchored_start: bool,
        is_anchored_end: bool,
        start: usize,
        matches: Vec<usize>,
    }

    struct Hir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl Hir {
        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let exprs = vec![
        Hir { anchored_start: true, anchored_end: true },
    ];

    let mut test_instance = TestSelf {
        compiled: Compiled {
            is_anchored_start: false,
            is_anchored_end: false,
            start: 0,
            matches: Vec::new(),
        },
    };

    let result = compile_many(test_instance, &exprs);
    assert!(result.is_err());
}


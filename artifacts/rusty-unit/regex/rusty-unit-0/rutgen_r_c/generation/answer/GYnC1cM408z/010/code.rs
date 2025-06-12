// Answer 0

fn test_compile_many_valid() -> Result<(), Error> {
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

    let mut compiler = Compiler::new()
        .size_limit(1000) 
        .bytes(false)
        .only_utf8(true)
        .dfa(false);

    let exprs = vec![
        TestExpr { is_anchored_start: false, is_anchored_end: false },
        TestExpr { is_anchored_start: false, is_anchored_end: false },
    ];

    let result = compiler.compile_many(&exprs);
    assert!(result.is_ok());
    
    Ok(())
}

#[should_panic]
fn test_compile_many_no_expr() {
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

    let mut compiler = Compiler::new().dfa(false);
    let exprs: Vec<TestExpr> = vec![];

    // This should panic due to exprs.len() being 0
    let _ = compiler.compile_many(&exprs);
}

fn test_compile_many_needs_dotstar() -> Result<(), Error> {
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

    let mut compiler = Compiler::new()
        .size_limit(1000) 
        .bytes(false)
        .only_utf8(true)
        .dfa(true); // Set DFA to true to trigger dotstar condition

    let exprs = vec![
        TestExpr { is_anchored_start: false, is_anchored_end: false },
        TestExpr { is_anchored_start: false, is_anchored_end: false },
    ];

    let result = compiler.compile_many(&exprs);
    assert!(result.is_ok());

    Ok(())
}

#[should_panic(expected = "panicked on a constraint")]
fn test_compile_many_capture_error() {
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

    let mut compiler = Compiler::new().dfa(false);
    
    let exprs = vec![
        TestExpr { is_anchored_start: false, is_anchored_end: false },
        TestExpr { is_anchored_start: true, is_anchored_end: true },
    ];

    // This should panic due to self.c_capture be returning an Err.
    let _ = compiler.compile_many(&exprs[0..1]); // only one expr, causes constraints to fail
}


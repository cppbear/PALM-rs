// Answer 0

fn test_compile_many_valid_case() {
    struct MockHir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockHir {
        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }
    
    let exprs = vec![
        MockHir { anchored_start: false, anchored_end: false },
        MockHir { anchored_start: false, anchored_end: false }
    ];

    let compiler = Compiler::new();

    let result = compiler.compile_many(&exprs);

    assert!(result.is_ok());
}

fn test_compile_many_with_dotstar_need() {
    struct MockHir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockHir {
        fn is_anchored_start(&self) -> bool {
            false // Needs dotstar to be true
        }

        fn is_anchored_end(&self) -> bool {
            false
        }
    }

    let exprs = vec![
        MockHir { anchored_start: false, anchored_end: false },
        MockHir { anchored_start: false, anchored_end: false }
    ];

    let mut compiler = Compiler::new();
    compiler.compiled.is_dfa = true; // Trigger dotstar requirement
    
    let result = compiler.compile_many(&exprs);

    assert!(result.is_err());
    if let Err(Error::Syntax(_)) = result {
        // The correct error type was returned
    } else {
        panic!("Expected Syntax error");
    }
}

fn test_compile_many_empty_capture() {
    struct MockHir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockHir {
        fn is_anchored_start(&self) -> bool {
            true
        }

        fn is_anchored_end(&self) -> bool {
            true
        }
    }

    let exprs = vec![
        MockHir { anchored_start: true, anchored_end: true },
        MockHir { anchored_start: true, anchored_end: true }
    ];

    let compiler = Compiler::new();

    let result = compiler.compile_many(&exprs);

    assert!(result.is_ok());
}

fn test_compile_many_single_expression() {
    struct MockHir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockHir {
        fn is_anchored_start(&self) -> bool {
            true
        }

        fn is_anchored_end(&self) -> bool {
            true
        }
    }

    let exprs: Vec<MockHir> = vec![
        MockHir { anchored_start: true, anchored_end: true }
    ];

    let compiler = Compiler::new();

    let result = compiler.compile_many(&exprs);

    assert!(result.is_err());
    if let Err(Error::Syntax(_)) = result {
        // The correct error type was returned
    } else {
        panic!("Expected Syntax error");
    }
}

fn test_compile_many_multiple_expressions() {
    struct MockHir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockHir {
        fn is_anchored_start(&self) -> bool {
            false
        }

        fn is_anchored_end(&self) -> bool {
            false
        }
    }

    let exprs = vec![
        MockHir { anchored_start: false, anchored_end: false },
        MockHir { anchored_start: false, anchored_end: false },
        MockHir { anchored_start: false, anchored_end: false },
    ];

    let compiler = Compiler::new();

    let result = compiler.compile_many(&exprs);

    assert!(result.is_ok());
}


// Answer 0

fn test_compile_one_no_dotstar() {
    struct DummyHir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl DummyHir {
        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let hir = DummyHir {
        anchored_start: false,
        anchored_end: false,
    };

    let compiler = Compiler::new();
    let result = compiler.compile_one(&hir);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert_eq!(program.start, 0);
    assert_eq!(program.captures.len(), 1);
}

fn test_compile_one_with_dotstar() {
    struct DummyHir {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl DummyHir {
        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let hir = DummyHir {
        anchored_start: false,
        anchored_end: false,
    };

    let mut compiler = Compiler::new().dfa(true);
    let result = compiler.compile_one(&hir);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert!(program.needs_dotstar());
    assert!(program.start > 0); // Ensure that a dotstar patch was added.
    assert_eq!(program.captures.len(), 1);
}


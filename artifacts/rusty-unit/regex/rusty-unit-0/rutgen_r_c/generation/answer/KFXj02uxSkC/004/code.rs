// Answer 0

#[test]
fn test_compile_one_needs_dotstar_true() {
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

    let expr = DummyHir {
        anchored_start: false,
        anchored_end: false,
    };

    let result = Compiler::new()
        .dfa(true)
        .compile_one(&expr);
    
    assert!(result.is_ok());
}

#[test]
fn test_compile_one_needs_dotstar_false() {
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

    let expr = DummyHir {
        anchored_start: true,
        anchored_end: false,
    };

    let result = Compiler::new()
        .dfa(true)
        .compile_one(&expr);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_compile_one_invalid_capture() {
    struct InvalidHir;

    impl InvalidHir {
        fn is_anchored_start(&self) -> bool {
            false
        }

        fn is_anchored_end(&self) -> bool {
            false
        }
    }

    let expr = InvalidHir;

    let result = Compiler::new()
        .dfa(true)
        .compile_one(&expr);
    
    assert!(result.is_err());
}


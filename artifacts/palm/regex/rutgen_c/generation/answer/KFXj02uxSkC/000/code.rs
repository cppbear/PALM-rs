// Answer 0

#[test]
fn test_compile_one_with_anchored_start() {
    struct MockExpr {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockExpr {
        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }
        
        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let expr = MockExpr {
        anchored_start: true,
        anchored_end: false,
    };
    
    let result = Compiler::new().compile_one(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_one_with_unanchored_start() {
    struct MockExpr {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockExpr {
        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }
        
        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let expr = MockExpr {
        anchored_start: false,
        anchored_end: false,
    };
    
    let result = Compiler::new().compile_one(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_one_fails_on_size_limit() {
    struct MockExpr;

    impl MockExpr {
        fn is_anchored_start(&self) -> bool {
            false
        }
        
        fn is_anchored_end(&self) -> bool {
            false
        }
    }
    
    let result = Compiler::new().size_limit(0).compile_one(&MockExpr);
    assert!(result.is_err());
}


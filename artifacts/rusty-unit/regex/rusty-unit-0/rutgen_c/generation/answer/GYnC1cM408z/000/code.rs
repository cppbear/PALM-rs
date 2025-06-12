// Answer 0

#[test]
fn test_compile_many_with_multiple_exprs() {
    struct TestHir {
        is_anchored_start: bool,
        is_anchored_end: bool,
    }
    
    impl TestHir {
        fn new(is_anchored_start: bool, is_anchored_end: bool) -> Self {
            TestHir {
                is_anchored_start,
                is_anchored_end,
            }
        }
        
        fn is_anchored_start(&self) -> bool {
            self.is_anchored_start
        }
        
        fn is_anchored_end(&self) -> bool {
            self.is_anchored_end
        }
    }
    
    let mut compiler = Compiler::new();
    
    let exprs = [
        TestHir::new(false, false),
        TestHir::new(true, true),
        TestHir::new(false, true),
    ];
    
    let result = compiler.compile_many(&exprs);
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.matches.len(), 3);
    assert_eq!(program.start, 0);
    assert!(program.is_anchored_start);
    assert!(program.is_anchored_end);
}

#[test]
fn test_compile_many_with_single_expr() {
    struct TestHir {
        is_anchored_start: bool,
        is_anchored_end: bool,
    }
    
    impl TestHir {
        fn new(is_anchored_start: bool, is_anchored_end: bool) -> Self {
            TestHir {
                is_anchored_start,
                is_anchored_end,
            }
        }
        
        fn is_anchored_start(&self) -> bool {
            self.is_anchored_start
        }
        
        fn is_anchored_end(&self) -> bool {
            self.is_anchored_end
        }
    }
    
    let mut compiler = Compiler::new();
    
    let exprs = [TestHir::new(false, false)];
    
    let result = std::panic::catch_unwind(|| {
        compiler.compile_many(&exprs)
    });
    
    assert!(result.is_err());
}

#[test]
fn test_compile_many_with_no_exprs() {
    let mut compiler = Compiler::new();
    let exprs: Vec<TestHir> = vec![];
    
    let result = std::panic::catch_unwind(|| {
        compiler.compile_many(&exprs)
    });
    
    assert!(result.is_err());
}


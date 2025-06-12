// Answer 0

#[test]
fn test_c_repeat_zero_or_more_greedy() {
    struct MockExpr;
    struct MockHir {
        kind: MockExpr,
    }

    impl MockHir {
        fn new() -> Self {
            Self { kind: MockExpr }
        }
    }

    let mut compiler = Compiler::new();
    let expr = MockHir::new();
    
    let result = compiler.c_repeat_zero_or_more(&expr, true);
    
    match result {
        Ok(patch) => {
            assert!(matches!(patch.hole, Hole::Many(_)));
            assert_eq!(patch.entry, 0);
        },
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

#[test]
fn test_c_repeat_zero_or_more_non_greedy() {
    struct MockExpr;
    struct MockHir {
        kind: MockExpr,
    }

    impl MockHir {
        fn new() -> Self {
            Self { kind: MockExpr }
        }
    }

    let mut compiler = Compiler::new();
    let expr = MockHir::new();
    
    let result = compiler.c_repeat_zero_or_more(&expr, false);
    
    match result {
        Ok(patch) => {
            assert!(matches!(patch.hole, Hole::Many(_)));
            assert_eq!(patch.entry, 0);
        },
        Err(_) => panic!("Expected Ok but got Err"),
    }
}


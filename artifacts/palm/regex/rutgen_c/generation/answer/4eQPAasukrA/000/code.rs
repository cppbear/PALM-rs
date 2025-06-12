// Answer 0

#[test]
fn test_c_repeat_range_exact() {
    struct DummyHir;
    
    impl DummyHir {
        fn new() -> Self {
            DummyHir {}
        }
    }
    
    let mut compiler = Compiler::new();
    let expr = DummyHir::new();
    let result = compiler.c_repeat_range(&expr, true, 3, 3);
    
    assert!(result.is_ok());
}

#[test]
fn test_c_repeat_range_min_less_than_max() {
    struct DummyHir;
    
    impl DummyHir {
        fn new() -> Self {
            DummyHir {}
        }
    }
    
    let mut compiler = Compiler::new();
    let expr = DummyHir::new();
    let result = compiler.c_repeat_range(&expr, false, 2, 5);
    
    assert!(result.is_ok());
}

#[test]
fn test_c_repeat_range_min_equals_max() {
    struct DummyHir;
    
    impl DummyHir {
        fn new() -> Self {
            DummyHir {}
        }
    }
    
    let mut compiler = Compiler::new();
    let expr = DummyHir::new();
    let result = compiler.c_repeat_range(&expr, true, 1, 1);
    
    assert!(result.is_ok());
}

#[should_panic]
fn test_c_repeat_range_overflow() {
    struct DummyHir;
    
    impl DummyHir {
        fn new() -> Self {
            DummyHir {}
        }
    }
    
    let mut compiler = Compiler::new();
    let expr = DummyHir::new();
    let result = compiler.c_repeat_range(&expr, true, u32::MAX, u32::MAX);
    
    // This should cause a panic due to overflow in u32_to_usize
    assert!(result.is_ok());
}


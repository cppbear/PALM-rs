// Answer 0

#[test]
fn test_c_concat_empty() {
    struct DummyHir;
    impl DummyHir {
        fn new() -> Self {
            DummyHir
        }
    }

    let mut compiler = Compiler::new();
    let result = compiler.c_concat(vec![]);

    match result {
        Ok(patch) => {
            assert_matches!(patch.hole, Hole::None);
            assert_eq!(patch.entry, compiler.insts.len());
        }
        Err(_) => panic!("Expected Ok but got an error"),
    }
}

#[test]
fn test_c_concat_single() {
    struct DummyHir;
    impl DummyHir {
        fn new() -> Self {
            DummyHir
        }
    }

    let mut compiler = Compiler::new();
    let exprs = vec![&DummyHir::new()]; // Single expression
    let result = compiler.c_concat(exprs.iter());

    match result {
        Ok(patch) => {
            assert_matches!(patch.hole, Hole::None);
            assert_eq!(patch.entry, compiler.insts.len()); // Entry should still be at the length after compilation
        }
        Err(_) => panic!("Expected Ok but got an error"),
    }
}


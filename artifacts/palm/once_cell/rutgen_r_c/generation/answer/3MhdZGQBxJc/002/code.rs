// Answer 0

#[test]
fn test_once_cell_debug_uninitialized() {
    struct TestType;

    let once_cell: OnceCell<TestType> = OnceCell::new();
    let result = format!("{:?}", once_cell);
    
    assert_eq!(result, "OnceCell(Uninit)");
}

#[test]
fn test_once_cell_debug_initialized() {
    struct TestType;

    let once_cell: OnceCell<TestType> = OnceCell::with_value(TestType);
    let result = format!("{:?}", once_cell);
    
    assert!(result.contains("OnceCell"));
}


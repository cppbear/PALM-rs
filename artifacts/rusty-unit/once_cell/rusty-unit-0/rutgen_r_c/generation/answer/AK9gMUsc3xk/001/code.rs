// Answer 0

#[test]
fn test_once_cell_new() {
    struct TestType;

    // Create a new OnceCell of type TestType
    let once_cell: OnceCell<TestType> = OnceCell::new();
    
    // Check the inner value is None
    assert_eq!(unsafe { (*once_cell.inner.get()) }, None);
}


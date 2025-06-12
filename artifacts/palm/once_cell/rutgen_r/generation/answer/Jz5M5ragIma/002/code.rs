// Answer 0

#[test]
fn test_wait_initialized() {
    use once_cell::sync::OnceCell;
    use std::sync::Arc;

    let cell = Arc::new(OnceCell::new());
    cell.set(92).unwrap(); // Initialize the cell

    // Since the cell is initialized, we can safely call wait and expect it to return the reference to the value
    let value: &u32 = cell.wait();
    assert_eq!(*value, 92);
}

#[test]
#[should_panic(expected = "Not initialized")]
fn test_wait_not_initialized() {
    use once_cell::sync::OnceCell;
    use std::sync::Arc;

    let cell = Arc::new(OnceCell::new());

    // The cell is not initialized, so calling wait should panic
    cell.wait();
}


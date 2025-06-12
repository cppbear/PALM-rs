// Answer 0

#[test]
fn test_wait_initialized() {
    use once_cell::sync::OnceCell;
    use std::sync::Arc;

    // Creating an OnceCell that is initialized
    let cell = Arc::new(OnceCell::new());
    cell.set(42).unwrap();

    // The wait should return a reference to the initialized value without blocking
    let value: &u32 = cell.wait();
    assert_eq!(*value, 42);
}

#[test]
#[should_panic]
fn test_wait_not_initialized() {
    use once_cell::sync::OnceCell;
    use std::sync::Arc;

    // Creating an OnceCell that is not initialized
    let cell = Arc::new(OnceCell::new());

    // The wait should panic since the value is not initialized
    let _value: &u32 = cell.wait();
}


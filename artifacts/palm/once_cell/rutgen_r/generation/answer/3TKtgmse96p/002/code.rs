// Answer 0

#[test]
fn test_get_or_try_init_with_initialized_cell() {
    use once_cell::sync::OnceCell;

    // Create an instance of OnceCell and initialize it
    let cell = OnceCell::new();

    // Initialize the cell using a closure that returns Ok
    let init_result = cell.get_or_try_init(|| Ok(42));
    assert_eq!(init_result, Ok(&42));
    
    // Confirm that the cell contains the initialized value
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_get_or_try_init_with_empty_cell() {
    use once_cell::sync::OnceCell;

    let cell: OnceCell<i32> = OnceCell::new();

    // First attempt to get value should fail as the cell is empty
    assert!(cell.get().is_none());

    // Attempt to initialize the cell with a closure that returns Ok
    let init_result = cell.get_or_try_init(|| Ok(73));
    assert_eq!(init_result, Ok(&73));
    
    // Confirm that the cell contains the initialized value
    assert_eq!(cell.get(), Some(&73));
}

#[test]
fn test_get_or_try_init_with_error() {
    use once_cell::sync::OnceCell;

    let cell: OnceCell<i32> = OnceCell::new();

    // Attempt to get value should fail as the cell is empty
    assert!(cell.get().is_none());

    // Attempt to initialize the cell with a closure that returns an error
    let init_result = cell.get_or_try_init(|| Err(()));
    assert_eq!(init_result, Err(()));
    
    // Confirm that the cell is still empty
    assert!(cell.get().is_none());
}

#[should_panic]
#[test]
fn test_get_or_try_init_panic_on_double_initialization() {
    use once_cell::sync::OnceCell;

    let cell: OnceCell<i32> = OnceCell::new();

    // Initialize the cell successfully
    let _ = cell.get_or_try_init(|| Ok(99));

    // Attempting to initialize the already initialized cell should trigger a panic
    let _ = cell.get_or_try_init(|| Ok(100));
}

#[test]
fn test_get_or_try_init_multiple_successive_calls() {
    use once_cell::sync::OnceCell;

    let cell: OnceCell<i32> = OnceCell::new();

    // Initialize the cell, should succeed
    let init_result = cell.get_or_try_init(|| Ok(57));
    assert_eq!(init_result, Ok(&57));
    
    // Repeated calls to get_or_try_init should return the same initialized value
    let second_result = cell.get_or_try_init(|| Ok(100));
    assert_eq!(second_result, Ok(&57));
    
    // Ensure cell contains the initialized value
    assert_eq!(cell.get(), Some(&57));
}


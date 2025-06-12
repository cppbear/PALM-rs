// Answer 0

#[test]
fn test_set_empty_cell() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();

    // Ensure the cell is empty initially
    assert!(CELL.get().is_none());

    // Try to set a value in the empty cell
    assert_eq!(CELL.set(42), Ok(()));
    assert_eq!(CELL.get(), Some(&42));
}

#[test]
fn test_set_non_empty_cell() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();

    // Set an initial value
    assert_eq!(CELL.set(42), Ok(()));
    assert_eq!(CELL.get(), Some(&42));

    // Try to set a value in the already filled cell
    assert_eq!(CELL.set(84), Err(84));
    assert_eq!(CELL.get(), Some(&42));
}

#[test]
fn test_set_concurrent_access() {
    use once_cell::sync::OnceCell;
    use std::thread;

    static CELL: OnceCell<i32> = OnceCell::new();

    // Ensure the cell is empty initially
    assert!(CELL.get().is_none());

    // Spawn a thread to set a value
    let handle = thread::spawn(|| {
        assert_eq!(CELL.set(100), Ok(()));
    });

    // Wait for the thread to finish
    handle.join().unwrap();
    
    // Ensure the main thread sees the set value
    assert_eq!(CELL.get(), Some(&100));
    
    // Trying to set another value should fail
    assert_eq!(CELL.set(200), Err(200));
    assert_eq!(CELL.get(), Some(&100));
}


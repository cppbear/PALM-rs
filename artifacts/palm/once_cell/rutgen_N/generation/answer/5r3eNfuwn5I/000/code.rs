// Answer 0

#[test]
fn test_set_empty_cell() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();

    assert!(CELL.get().is_none());
    assert_eq!(CELL.set(42), Ok(()));
    assert_eq!(CELL.get(), Some(&42));
}

#[test]
fn test_set_non_empty_cell() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();

    assert_eq!(CELL.set(42), Ok(()));
    assert_eq!(CELL.set(99), Err(99));
    assert_eq!(CELL.get(), Some(&42));
}

#[test]
fn test_set_multithreaded() {
    use once_cell::sync::OnceCell;
    use std::thread;

    static CELL: OnceCell<i32> = OnceCell::new();

    assert!(CELL.get().is_none());
    
    let handle = thread::spawn(|| {
        assert_eq!(CELL.set(100), Ok(()));
    });
    
    handle.join().unwrap();
    
    assert_eq!(CELL.set(200), Err(200));
    assert_eq!(CELL.get(), Some(&100));
}

#[test]
#[should_panic]
fn test_set_panic_on_invalid() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();
    
    // This method should not panic but should be covered by separate logic if needed.
    assert_eq!(CELL.set(50), Ok(()));
    assert_eq!(CELL.set(50), Err(50));
    // Attempting to set again should not panic but will fail gracefully; removed unnecessary panic.
}


// Answer 0

#[test]
fn test_set_cell_empty() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();

    // Assert the cell is initially empty
    assert!(CELL.get().is_none());
    
    // Set a value to the empty cell and expect Ok(())
    assert_eq!(CELL.set(42), Ok(()));
    assert_eq!(CELL.get(), Some(&42));
}

#[test]
fn test_set_cell_full() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();

    // Set a value to the empty cell
    assert_eq!(CELL.set(42), Ok(()));
    
    // Try to set a new value to the now full cell and expect Err(value)
    assert_eq!(CELL.set(62), Err(62));
    assert_eq!(CELL.get(), Some(&42));
}

#[test]
fn test_set_cell_multiple_threads() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();

    // Set a value in one thread
    let handle = std::thread::spawn(|| {
        assert_eq!(CELL.set(99), Ok(()));
    });
    handle.join().unwrap();

    // The cell is now full, so setting another value should return Err()
    assert_eq!(CELL.set(100), Err(100));
    assert_eq!(CELL.get(), Some(&99));
}


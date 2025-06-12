// Answer 0

#[test]
fn test_set_when_cell_is_full() {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<i32> = OnceCell::new();

    // First we set the cell with a value
    assert_eq!(CELL.set(42), Ok(())); // This should succeed first

    // Now we attempt to set it again, expecting an Err with the value
    assert_eq!(CELL.set(62), Err(62)); // This should return Err(62) since the cell is now full
    assert_eq!(CELL.get(), Some(&42)); // Confirm that the cell still contains the first value
}


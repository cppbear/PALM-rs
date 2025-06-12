// Answer 0

#[test]
fn test_get_or_try_init_with_value_existing() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    // Initialize the cell with a valid value first
    let _ = cell.get_or_try_init(|| -> Result<i32, ()> { Ok(42) });

    // Now that the cell has a value, this should return Ok without calling the function
    assert_eq!(cell.get_or_try_init(|| -> Result<i32, ()> { 
        panic!("This should not be called because the cell is initialized"); 
    }), Ok(&42));
}

#[test]
fn test_get_or_try_init_with_error() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();

    // When the cell is empty, calling with a function that returns an error should yield that error
    let result: Result<&i32, ()> = cell.get_or_try_init(|| Err(()));
    assert_eq!(result, Err(()));
    assert!(cell.get().is_none()); // The cell should still be empty
}


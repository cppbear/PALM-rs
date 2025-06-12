// Answer 0

#[test]
fn test_try_insert_ok_case() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    // Trying to insert a value when the cell is empty should return Ok with the inserted value
    assert_eq!(cell.try_insert(42), Ok(&42));
    assert!(cell.get().is_some());
}

#[test]
fn test_try_insert_err_case() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    // Insert the first value
    assert_eq!(cell.try_insert(42), Ok(&42));

    // Now trying to insert a second value should return Err
    assert_eq!(cell.try_insert(84), Err((&42, 84)));
    assert!(cell.get().is_some());
}


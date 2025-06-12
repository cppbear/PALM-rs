// Answer 0

#[test]
fn test_set_empty_cell() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert!(cell.get().is_none());
    
    // Test setting a value in the empty cell
    assert_eq!(cell.set(42), Ok(()));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_set_full_cell() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert_eq!(cell.set(100), Ok(()));  // First set, should succeed
    assert_eq!(cell.set(200), Err(200)); // Second set, should fail and return the value
    assert_eq!(cell.get(), Some(&100));  // Cell should contain the first value
}

#[test]
fn test_set_with_different_types() {
    use once_cell::unsync::OnceCell;

    // Testing with a different type
    let cell = OnceCell::new();
    assert_eq!(cell.set("Hello"), Ok(()));
    assert_eq!(cell.get(), Some(&"Hello"));
}


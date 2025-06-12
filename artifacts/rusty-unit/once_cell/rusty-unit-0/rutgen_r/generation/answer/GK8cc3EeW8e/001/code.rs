// Answer 0

#[test]
fn test_into_inner_empty_cell() {
    use once_cell::unsync::OnceCell;

    let cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_set_value() {
    use once_cell::unsync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
}

#[test]
fn test_into_inner_after_multiple_set() {
    use once_cell::unsync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("world".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("world".to_string()));

    // Attempt to set another value after the cell is already set
    let result = cell.set("another value".to_string());
    assert!(result.is_err()); // Expecting an error since the OnceCell is already occupied
}


// Answer 0

#[test]
fn test_once_cell_into_inner_empty() {
    use once_cell::sync::OnceCell;

    let cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_once_cell_into_inner_with_value() {
    use once_cell::sync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
}

#[test]
fn test_once_cell_into_inner_after_set() {
    use once_cell::sync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("rust".to_string()).unwrap();
    let inner = cell.into_inner();
    assert_eq!(inner, Some("rust".to_string()));
}

#[test]
#[should_panic]
fn test_once_cell_into_inner_multiple_set() {
    use once_cell::sync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("first".to_string()).unwrap();
    let _ = cell.into_inner(); // Consuming the cell
    
    // This should panic since the cell is already consumed.
    cell.set("second".to_string()).unwrap();
}


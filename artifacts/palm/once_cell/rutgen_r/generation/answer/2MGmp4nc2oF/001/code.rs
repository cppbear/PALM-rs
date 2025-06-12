// Answer 0

#[test]
fn test_into_inner_empty_cell() {
    use once_cell::sync::OnceCell;

    let cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_non_empty_cell() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
}

#[test]
fn test_into_inner_multiple_set_calls() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    cell.set("first".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("first".to_string()));

    let cell = OnceCell::new();
    cell.set("second".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("second".to_string()));
}

#[test]
fn test_into_inner_after_panic() {
    use once_cell::sync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("valid".to_string()).unwrap();
    let _ = cell.into_inner(); // This should not panic.
    
    let result = std::panic::catch_unwind(|| {
        cell.set("another".to_string()).unwrap();
    });

    assert!(result.is_err());
    assert_eq!(cell.into_inner(), Some("valid".to_string()));
}


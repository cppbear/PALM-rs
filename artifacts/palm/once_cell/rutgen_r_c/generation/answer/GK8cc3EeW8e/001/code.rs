// Answer 0

#[test]
fn test_into_inner_empty() {
    let cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_with_value() {
    let cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
}

#[test]
fn test_into_inner_after_take() {
    let cell = OnceCell::new();
    cell.set("world".to_string()).unwrap();
    assert_eq!(cell.take(), Some("world".to_string()));
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_multiple_set() {
    let cell = OnceCell::new();
    cell.set("first".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("first".to_string()));
    cell.set("second".to_string()).unwrap(); // This should not panic, and just overwrite
    assert_eq!(cell.into_inner(), Some("second".to_string()));
}


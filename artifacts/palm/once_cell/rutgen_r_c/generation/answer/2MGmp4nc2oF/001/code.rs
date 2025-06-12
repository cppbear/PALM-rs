// Answer 0

#[test]
fn test_into_inner_empty() {
    let cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_with_value() {
    let mut cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
}

#[test]
fn test_into_inner_after_take() {
    let mut cell = OnceCell::new();
    cell.set("world".to_string()).unwrap();
    let taken_value = cell.take();
    assert_eq!(taken_value, Some("world".to_string()));
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_with_different_type() {
    let mut cell = OnceCell::new();
    cell.set(42).unwrap();
    assert_eq!(cell.into_inner(), Some(42));
}

#[should_panic]
fn test_into_inner_after_set_fails() {
    let cell = OnceCell::new();
    let _ = cell.set("fail".to_string()).unwrap_err(); // Ensure set fails if previously set
    let _ = cell.into_inner(); // This will panic since we should not access after failed set
}


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
    cell.set("hello".to_string()).unwrap();
    cell.take();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_multiple_calls() {
    let mut cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
    assert_eq!(cell.into_inner(), None); // after moved, should be None
}


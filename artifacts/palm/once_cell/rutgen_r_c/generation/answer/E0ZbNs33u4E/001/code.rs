// Answer 0

#[test]
fn test_take_not_initialized() {
    let mut cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.take(), None);
}

#[test]
fn test_take_initialized() {
    let mut cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.take(), Some("hello".to_string()));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_take_multiple_times() {
    let mut cell = OnceCell::new();
    cell.set(42).unwrap();
    assert_eq!(cell.take(), Some(42));
    assert_eq!(cell.take(), None);
}

#[test]
fn test_take_with_different_types() {
    let mut cell = OnceCell::new();
    cell.set(vec![1, 2, 3]).unwrap();
    assert_eq!(cell.take(), Some(vec![1, 2, 3]));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_take_after_reset() {
    let mut cell = OnceCell::new();
    cell.set("example".to_string()).unwrap();
    assert_eq!(cell.take(), Some("example".to_string()));
    cell = OnceCell::new(); // Reset
    assert_eq!(cell.take(), None);
}


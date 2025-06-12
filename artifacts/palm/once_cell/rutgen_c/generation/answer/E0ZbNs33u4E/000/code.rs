// Answer 0

#[test]
fn test_take_uninitialized() {
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
fn test_take_multiple_calls() {
    let mut cell = OnceCell::new();
    cell.set(42).unwrap();
    assert_eq!(cell.take(), Some(42));
    assert_eq!(cell.get(), None);
    assert_eq!(cell.take(), None);
}

#[test]
fn test_take_after_double_initialization() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(100).unwrap();
    assert_eq!(cell.take(), Some(100));
    cell = OnceCell::new();
    assert_eq!(cell.take(), None);
}


// Answer 0

#[test]
fn test_take_initially_uninitialized() {
    let mut cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.take(), None);
}

#[test]
fn test_take_after_set() {
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
fn test_take_with_empty_cell() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    assert_eq!(cell.take(), None);
    cell.set(99).unwrap();
    assert_eq!(cell.take(), Some(99));
    assert_eq!(cell.take(), None);
}


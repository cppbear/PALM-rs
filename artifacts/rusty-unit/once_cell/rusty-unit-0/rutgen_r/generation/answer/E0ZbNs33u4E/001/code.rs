// Answer 0

#[test]
fn test_once_cell_take_uninitialized() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.take(), None);
}

#[test]
fn test_once_cell_take_initialized() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.take(), Some("hello".to_string()));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_once_cell_take_multiple_calls() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    assert_eq!(cell.take(), Some(42));
    assert_eq!(cell.get(), None);
    assert_eq!(cell.take(), None);
}

#[test]
#[should_panic]
fn test_once_cell_take_after_reinitialization() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(100).unwrap();
    let _ = cell.take();
    cell = OnceCell::new(); // reinitializing
    let _ = cell.take(); // This should not panic but returns None
}


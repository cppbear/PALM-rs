// Answer 0

#[test]
fn test_once_cell_take_uninitialized() {
    use once_cell::unsync::OnceCell;

    let mut cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.take(), None);
}

#[test]
fn test_once_cell_take_initialized() {
    use once_cell::unsync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.take(), Some("hello".to_string()));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_once_cell_take_multiple_times() {
    use once_cell::unsync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    assert_eq!(cell.take(), Some(42));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_once_cell_take_after_reinitialization() {
    use once_cell::unsync::OnceCell;

    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("once".to_string()).unwrap();
    assert_eq!(cell.take(), Some("once".to_string()));
    cell = OnceCell::new();
    assert_eq!(cell.take(), None);
}

#[test]
fn test_once_cell_take_empty_after_restart() {
    use once_cell::unsync::OnceCell;

    let mut cell: OnceCell<i32> = OnceCell::new();
    cell.set(100).unwrap();
    assert_eq!(cell.take(), Some(100));
    cell = OnceCell::new();
    assert_eq!(cell.take(), None);
}


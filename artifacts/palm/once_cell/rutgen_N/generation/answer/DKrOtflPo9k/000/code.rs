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

    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.take(), Some("hello".to_string()));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_once_cell_take_multiple() {
    use once_cell::unsync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(92).unwrap();
    let value = cell.take().unwrap();
    assert_eq!(value, 92);
    assert_eq!(cell.get(), None);
}


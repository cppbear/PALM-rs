// Answer 0

#[test]
fn test_take_uninitialized_once_cell() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.take(), None);
}

#[test]
fn test_take_initialized_once_cell() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.take(), Some("hello".to_string()));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_take_multiple_times() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(92).unwrap();
    assert_eq!(cell.take(), Some(92));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_take_after_reinitialization() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(10).unwrap();
    cell.take(); // take the value
    let mut cell = OnceCell::new(); // reinitialize
    cell.set(20).unwrap();
    assert_eq!(cell.take(), Some(20));
}


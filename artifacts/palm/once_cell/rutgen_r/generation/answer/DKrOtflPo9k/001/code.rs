// Answer 0

#[test]
fn test_take_initially_uninitialized() {
    let mut cell: once_cell::unsync::OnceCell<String> = once_cell::unsync::OnceCell::new();
    assert_eq!(cell.take(), None);
}

#[test]
fn test_take_after_set() {
    let mut cell: once_cell::unsync::OnceCell<String> = once_cell::unsync::OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.take(), Some("hello".to_string()));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_take_after_multiple_sets() {
    let mut cell: once_cell::unsync::OnceCell<u32> = once_cell::unsync::OnceCell::new();
    cell.set(100).unwrap();
    assert_eq!(cell.take(), Some(100));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_take_with_reset() {
    let mut cell: once_cell::unsync::OnceCell<String> = once_cell::unsync::OnceCell::new();
    cell.set("test".to_string()).unwrap();
    let _ = cell.take(); // Take the value first
    cell.set("another".to_string()).unwrap(); // Set another value
    assert_eq!(cell.take(), Some("another".to_string())); // Take new value
    assert_eq!(cell.get(), None);
}

#[test]
fn test_take_with_different_types() {
    let mut cell: once_cell::unsync::OnceCell<f64> = once_cell::unsync::OnceCell::new();
    cell.set(3.14).unwrap();
    assert_eq!(cell.take(), Some(3.14));
    assert_eq!(cell.get(), None);
}


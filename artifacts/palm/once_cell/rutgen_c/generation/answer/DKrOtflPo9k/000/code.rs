// Answer 0

#[test]
fn test_take_not_initialized() {
    let mut cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.take(), None);
}

#[test]
fn test_take_initialized() {
    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.take(), Some("hello".to_string()));
    assert_eq!(cell.get(), None);
}

#[test]
fn test_take_multiple_calls() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    
    let taken_value = cell.take();
    assert_eq!(taken_value, Some(42));
    assert_eq!(cell.get(), None);

    // After taking the value, it should be uninitialized
    assert_eq!(cell.take(), None);
}


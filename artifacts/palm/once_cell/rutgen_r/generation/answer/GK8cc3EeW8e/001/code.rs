// Answer 0

#[test]
fn test_into_inner_empty_cell() {
    use once_cell::unsync::OnceCell;

    let cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_non_empty_cell() {
    use once_cell::unsync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
}

#[test]
#[should_panic]
fn test_into_inner_panic_after_set() {
    use once_cell::unsync::OnceCell;

    let mut cell = OnceCell::new();
    cell.set("world".to_string()).unwrap();
    let _ = cell.into_inner(); // This should not panic
    let _ = cell.into_inner(); // This should panic as we try to access again
}


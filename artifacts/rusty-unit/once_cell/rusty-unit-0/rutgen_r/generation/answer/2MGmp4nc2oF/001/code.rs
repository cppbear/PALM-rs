// Answer 0

#[test]
fn test_once_cell_empty() {
    use once_cell::sync::OnceCell;

    let cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_once_cell_set_value() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
}

#[test]
fn test_once_cell_set_multiple_times() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    cell.set("first".to_string()).unwrap();
    // The second set should panic if attempted, so we'll skip this.
    assert_eq!(cell.into_inner(), Some("first".to_string()));
}

#[test]
#[should_panic]
fn test_once_cell_panic_on_multiple_set() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    cell.set("value1".to_string()).unwrap();
    cell.set("value2".to_string()).unwrap(); // This should trigger a panic
}


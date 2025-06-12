// Answer 0

#[test]
fn test_get_empty() {
    struct TestType;
    let cell: OnceCell<TestType> = OnceCell::new();
    assert_eq!(cell.get(), None);
}

#[test]
fn test_get_with_value() {
    struct TestType;
    let cell: OnceCell<TestType> = OnceCell::with_value(TestType);
    assert!(cell.get().is_some());
}

#[test]
fn test_get_after_set() {
    struct TestType;
    let cell: OnceCell<TestType> = OnceCell::new();
    let _ = cell.set(TestType);
    assert!(cell.get().is_some());
}


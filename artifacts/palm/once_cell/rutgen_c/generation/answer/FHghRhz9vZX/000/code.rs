// Answer 0

#[test]
fn test_with_value_creates_initialized_once_cell() {
    struct TestStruct {
        value: i32,
    }

    let cell = OnceCell::with_value(TestStruct { value: 42 });
    assert!(cell.get().is_some());
    assert_eq!(cell.get().unwrap().value, 42);
}

#[test]
fn test_with_value_creates_cell_with_none_when_no_value() {
    struct TestStruct;

    let cell: OnceCell<TestStruct> = OnceCell::new();
    assert!(cell.get().is_none());
}


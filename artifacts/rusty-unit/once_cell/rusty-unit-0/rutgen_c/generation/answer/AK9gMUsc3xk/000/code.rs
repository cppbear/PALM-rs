// Answer 0

#[test]
fn test_once_cell_new() {
    struct TestStruct;

    let cell: OnceCell<TestStruct> = OnceCell::new();
    assert_eq!(cell.get(), None);
}

#[test]
fn test_once_cell_set_and_get() {
    struct TestStruct(i32);

    let cell: OnceCell<TestStruct> = OnceCell::new();
    assert_eq!(cell.get(), None);
    
    cell.set(TestStruct(42)).unwrap();
    assert_eq!(cell.get(), Some(&TestStruct(42)));
}

#[test]
fn test_once_cell_try_insert() {
    struct TestStruct(i32);

    let cell: OnceCell<TestStruct> = OnceCell::new();
    assert_eq!(cell.try_insert(TestStruct(10)).is_ok(), true);
    assert_eq!(cell.get(), Some(&TestStruct(10)));

    let result = cell.try_insert(TestStruct(20));
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_once_cell_take() {
    struct TestStruct(i32);

    let mut cell: OnceCell<TestStruct> = OnceCell::new();
    assert_eq!(cell.take(), None);

    cell.set(TestStruct(30)).unwrap();
    assert_eq!(cell.take(), Some(TestStruct(30)));
    assert_eq!(cell.take(), None);
}

#[test]
fn test_once_cell_into_inner() {
    struct TestStruct(i32);

    let cell: OnceCell<TestStruct> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);

    let mut cell_with_value: OnceCell<TestStruct> = OnceCell::new();
    cell_with_value.set(TestStruct(50)).unwrap();
    assert_eq!(cell_with_value.into_inner(), Some(TestStruct(50)));
}


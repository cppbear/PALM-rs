// Answer 0

#[test]
fn test_try_insert_success() {
    struct TestStruct;
    
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(TestStruct), Ok(&TestStruct));
    assert!(cell.get().is_some());
}

#[test]
fn test_try_insert_fail() {
    struct TestStruct;

    let cell = OnceCell::new();
    assert_eq!(cell.try_insert(TestStruct), Ok(&TestStruct));

    let new_value = TestStruct;
    let result = cell.try_insert(new_value);
    assert!(result.is_err());

    if let Err((old, new)) = result {
        assert_eq!(old, &TestStruct);
        // Check that new_value was attempted to be inserted
        assert_eq!(new, new_value);
    }
}


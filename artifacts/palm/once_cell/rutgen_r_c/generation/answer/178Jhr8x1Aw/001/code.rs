// Answer 0

#[test]
fn test_try_insert_with_err() {
    struct TestStruct {
        data: i32,
    }

    impl TestStruct {
        fn new(data: i32) -> Self {
            TestStruct { data }
        }
    }

    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    // First insert should succeed
    assert_eq!(cell.try_insert(TestStruct::new(42)), Ok(&TestStruct::new(42)));

    // Second insert should fail with Err
    let insert_result = cell.try_insert(TestStruct::new(25));
    assert!(insert_result.is_err());

    // Extract the value from the Err result
    let (existing_value, new_value) = insert_result.unwrap_err();
    assert_eq!(existing_value.data, 42);
    assert_eq!(new_value.data, 25);
}


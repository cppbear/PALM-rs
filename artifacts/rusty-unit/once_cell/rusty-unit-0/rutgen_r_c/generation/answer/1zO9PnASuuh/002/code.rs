// Answer 0

#[test]
fn test_get_or_try_init_success_initial_empty() {
    struct TestStruct {
        value: i32,
    }
    impl TestStruct {
        fn new(value: i32) -> Self {
            TestStruct { value }
        }
    }

    let cell = OnceCell::new();
    let value = cell.get_or_try_init(|| Ok(TestStruct::new(42)));
    assert_eq!(value, Ok(&TestStruct::new(42)));
    assert!(cell.get().is_some());
}

#[test]
fn test_get_or_try_init_success_with_existing_value() {
    struct TestStruct {
        value: i32,
    }
    impl TestStruct {
        fn new(value: i32) -> Self {
            TestStruct { value }
        }
    }

    let cell = OnceCell::new();
    let _ = cell.set(TestStruct::new(42)).unwrap();
    let value = cell.get_or_try_init(|| Ok(TestStruct::new(13)));
    assert_eq!(value, Ok(&TestStruct::new(42)));
    assert!(cell.get().is_some());
}

#[test]
#[should_panic]
fn test_get_or_try_init_reentrant_init() {
    let cell = OnceCell::new();
    let _ = cell.get_or_try_init(|| cell.get_or_try_init(|| Ok(())));
}

#[test]
fn test_get_or_try_init_failure() {
    let cell = OnceCell::new();
    let value = cell.get_or_try_init(|| Err(()));
    assert_eq!(value, Err(()));
    assert!(cell.get().is_none());
}

#[test]
fn test_get_or_try_init_success_after_failure() {
    struct TestStruct {
        value: i32,
    }
    impl TestStruct {
        fn new(value: i32) -> Self {
            TestStruct { value }
        }
    }

    let cell = OnceCell::new();
    let _ = cell.get_or_try_init(|| Err(()));
    let value = cell.get_or_try_init(|| Ok(TestStruct::new(99)));
    assert_eq!(value, Ok(&TestStruct::new(99)));
    assert!(cell.get().is_some());
}


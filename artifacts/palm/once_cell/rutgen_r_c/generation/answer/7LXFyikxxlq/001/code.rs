// Answer 0

#[test]
fn test_get_or_try_init_with_existing_value() {
    struct TestData {
        value: i32,
    }

    let once_ref = OnceRef::new();
    let test_value = TestData { value: 42 };

    // Set the value first to satisfy the condition that self.get() matches Some(val)
    once_ref.set(&test_value).unwrap();

    let result: Result<&TestData, ()> = once_ref.get_or_try_init(|| Err(()));

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_get_or_try_init_with_initialization_failing() {
    struct TestData {
        value: i32,
    }

    let once_ref = OnceRef::new();
    let test_value = TestData { value: 42 };

    // Set the value first to satisfy the condition that self.get() matches Some(val)
    once_ref.set(&test_value).unwrap();

    let result: Result<&TestData, ()> = once_ref.get_or_try_init(|| {
        // This initialization function deliberately fails
        Err(())
    });

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().value, 42);
}

